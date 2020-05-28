package keys

import (
	"context"

	"github.com/jmoiron/sqlx"
	"gitlab.com/bloom42/gobox/uuid"
)

func FindGroupMasterKey(ctx context.Context, tx *sqlx.Tx, groupID uuid.UUID) ([]byte, error) {
	return nil, nil
}
