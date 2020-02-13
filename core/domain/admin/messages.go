package admin

import (
	"gitlab.com/bloom42/bloom/core/api/model"
)

type DashboardData struct {
	Metadata *model.BloomMetadata `json:"metadata"`
}
