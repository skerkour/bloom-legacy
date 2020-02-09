package admin

import (
	"gitlab.com/bloom42/bloom/core/api/model"
)

type DashboardData struct {
	ServerVersion *model.ServerVersion `json:"serverVersion"`
}
