package groups

import (
	"context"

	"gitlab.com/bloom42/bloom/core/api"
	"gitlab.com/bloom42/bloom/core/api/model"
	"gitlab.com/bloom42/gobox/graphql"
)

func FetchMyInvitations() (*model.User, error) {
	client := api.Client()

	var resp struct {
		Me *model.User `json:"me"`
	}
	req := graphql.NewRequest(`
	query {
		me {
			groupInvitations {
				nodes {
					id
					ephemeralPublicKey
					signature
					encryptedMasterKey
					group {
						id
						name
						description
					}
					inviter {
						username
						displayName
						publicKey
					}
				}
				totalCount
			}
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.Me, err
}
