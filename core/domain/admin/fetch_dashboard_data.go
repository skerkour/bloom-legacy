package admin

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/libs/graphql-go"
)

func FetchDashboardData() (DashboardData, error) {
	client := api.Client()
	var ret DashboardData

	req := graphql.NewRequest(`
	query {
		metadata {
			os
			arch
			version
			gitCommit
		}
	}
	`)

	err := client.Do(context.Background(), req, &ret)

	return ret, err
}
