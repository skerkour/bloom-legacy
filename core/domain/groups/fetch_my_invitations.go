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
			invitations {
				nodes {
					group {
						name
						description
					}
					inviter {
						username
						displayName
						publicKey
					}
					ephemeralPublicKey
					signature
					encryptedMasterKey
					encryptedMasterKeySignature
				}
				totalCount
			}
		}
	}
	`)

	err := client.Do(context.Background(), req, &resp)

	return resp.Me, err
}
