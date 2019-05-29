#!/bin/bash -e

# PostgreSQL settings
DATABASE_URL=$DATABASE_URL

# OpenSSL settings
ENCRYPTION_KEY=$ENCRYPTION_KEY
ENCRYPTION_CIPHER=$ENCRYPTION_CIPHER

# Amazon settings
AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION
AWS_S3_BUCKET=$AWS_S3_BUCKET
AWS_S3_PATH=$AWS_S3_PATH

# backup settings
BACKUP_INTERVAL=86400
BACKUP_NAME=backup

backup_and_stream_to_s3() {

while true
  do
    BACKUP="${BACKUP_NAME}_`date +"%Y-%m-%d_%H-%M"`${EXTENSION}"
    echo "Set backup file name to: $BACKUP"
    echo "Starting database backup.."
    pg_dump $DATABASE_URL | gzip | openssl enc -$ENCRYPTION_CIPHER -salt -k $ENCRYPTION_KEY | aws s3 cp - "${AWS_S3_BUCKET}${AWS_S3_PATH}/${BACKUP}"
    # use the following command to decrypt
    # openssl enc -$ENCRYPTION_CIPHER -d -in ${IN_FILE} -out ${IN_FILE} -k $ENCRYPTION_KEY
    echo "Backup finished! Sleeping ${TIMEOUT}s"
    sleep $TIMEOUT
  done

}

backup_and_stream_to_s3

wait
