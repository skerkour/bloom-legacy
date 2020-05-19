package api

import (
	"context"
	"fmt"

	"gitlab.com/bloom42/bloom/core/version"
	"gitlab.com/bloom42/gobox/graphql"
	"gitlab.com/bloom42/gobox/uuid"
)

type ApiClient struct {
	graphql   *graphql.Client
	SessionID *uuid.UUID
	Token     *string
}

var client *ApiClient

func Client() *ApiClient {
	if client == nil {
		client = &ApiClient{
			graphql: graphql.NewClient(SERVER_URL + "/api/graphql"),
		}
	}
	return client
}

func (c *ApiClient) Authenticate(sessionID uuid.UUID, token string) {
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
	req.Header.Set("User-Agent", fmt.Sprintf("com.bloom42.bloom.core/%s", version.Version))
	if c.Token != nil {
		req.Header.Add("authorization", fmt.Sprintf("Basic %s", *c.Token))
	}
	return c.graphql.Do(ctx, req, resp)
}
