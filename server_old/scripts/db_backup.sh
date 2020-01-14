#!/bin/bash -e

# PostgreSQL settings
: ${DATABASE_URL:?"DATABASE_URL not specified"}


# OpenSSL settings
ENCRYPTION_CIPHER=${ENCRYPTION_CIPHER:-"aes-256-gcm"} # old: aes-256-cbc
: ${ENCRYPTION_KEY:?"ENCRYPTION_KEY not specified"}


# Amazon settings
: ${AWS_ACCESS_KEY_ID:?"AWS_ACCESS_KEY_ID not specified"}
: ${AWS_SECRET_ACCESS_KEY:?"AWS_SECRET_ACCESS_KEY not specified"}
# AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION
: ${AWS_S3_BUCKET:?"AWS_S3_BUCKET not specified"}
AWS_S3_PATH=${AWS_S3_PATH:-"/backup/db"}

# backup settings
BACKUP_INTERVAL=${BACKUP_INTERVAL:-86400}
BACKUP_NAME=${BACKUP_NAME:-backup}
BACKUP_EXTENSION=${BACKUP_EXTENSION:-.sql.gz.enc}

backup_and_stream_to_s3() {

while true
  do
    BACKUP="${BACKUP_NAME}_`date +"%Y-%m-%d_%H-%M"`${BACKUP_EXTENSION}"
    echo "Set backup file name to: $BACKUP_NAME"
    echo "Starting database backup.."
    pg_dump $DATABASE_URL | gzip | openssl enc -$ENCRYPTION_CIPHER -salt -k $ENCRYPTION_KEY | aws s3 cp - s3://"${AWS_S3_BUCKET}${AWS_S3_PATH}/${BACKUP}"
    # use the following command to decrypt
    # cat ${BACKUP} | openssl enc -$ENCRYPTION_CIPHER -d -k $ENCRYPTION_KEY | gzip -d > db.sql
    echo "Backup finished! Sleeping ${BACKUP_INTERVAL}s"
    sleep $BACKUP_INTERVAL
  done

}

backup_and_stream_to_s3

wait
