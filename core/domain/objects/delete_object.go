package objects

import (
	"context"

	"github.com/jmoiron/sqlx"
)

func DeleteObject(ctx context.Context, tx *sqlx.Tx, id []byte) error {
	// TODO
	return nil
}
