package admin

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/messages"
	"gitlab.com/bloom42/lily/graphql"
)

func FetchDashboardData() (messages.DashboardData, error) {
	client := api.Client()
	var ret messages.DashboardData

	req := graphql.NewRequest(`
	query {
		metadata {
			os
			arch
			version
			gitCommit
		}
		users {
			totalCount
		}
	}
	`)

	err := client.Do(context.Background(), req, &ret)

	return ret, err
}
