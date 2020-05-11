package users

import (
	"context"
	"encoding/base64"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/bloom/core/domain/preferences"
)

func FindMasterKey(ctx context.Context, tx *sqlx.Tx) ([]byte, error) {
	encoded, err := preferences.Get(context.Background(), tx, MASTER_KEY_KEY)
	if err != nil {
		return nil, err
	}

	masterKey, err := base64.StdEncoding.DecodeString(encoded)
	return masterKey, err
}
