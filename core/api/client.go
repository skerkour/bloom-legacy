package api

import (
	"context"
	"fmt"

	"gitlab.com/bloom42/bloom/common/consts"
	"gitlab.com/bloom42/libs/graphql-go"
)

type ApiClient struct {
	graphql   *graphql.Client
	SessionID *string
	Token     *string
}

var client *ApiClient

func Client() *ApiClient {
	if client == nil {
		client = &ApiClient{
			graphql: graphql.NewClient(consts.API_BASE_URL + "/graphql"),
		}
	}
	return client
}

func (c *ApiClient) Authenticate(sessionID, token string) {
	c.SessionID = &sessionID
	c.Token = &token
}

func (c *ApiClient) Deauthenticate() {
	c.SessionID = nil
	c.Token = nil
}

func (c *ApiClient) IsAuthenticated() bool {
	if c.Token == nil {
		return false
	} else {
		return true
	}
}

func (c *ApiClient) Do(ctx context.Context, req *graphql.Request, resp interface{}) error {
	if c.Token != nil {
		req.Header.Add("authorization", fmt.Sprintf("Basic %s", *c.Token))
	}
	return c.graphql.Do(ctx, req, resp)
}
