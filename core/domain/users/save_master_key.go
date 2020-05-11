package users

import (
	"context"
	"encoding/base64"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

func SaveMasterKey(ctx context.Context, tx *sqlx.Tx, masterKey []byte) error {
	encoded := base64.StdEncoding.EncodeToString(masterKey)
	return preferences.Set(context.Background(), tx, MASTER_KEY_KEY, encoded)
}
