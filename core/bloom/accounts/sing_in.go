package accounts

import (
	"context"
	"gitlab.com/bloom42/bloom/core/rpc/accounts"
	"net/http"
)

/*

pub fn sign_in(input: auth::GuiSignIn) -> Result<Message, BloomError> {
    let auth_key = derive_auth_key(&input.username, &input.password)?;

    let message: Message = auth::SignIn {
        username: input.username,
        auth_key: base64::encode(&auth_key),
    }
    .into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}
*/
func SignIn(params SignInParams) (Session, error) {
	client := accounts.NewAccountsProtobufClient("http://localhost:8080", &http.Client{})

	authKey := deriveAuthKey([]byte(params.Username), []byte(params.Password))
	rpcParams := accounts.SignInParams{
		Username: params.Username,
		AuthKey:  authKey,
	}

	session, err := client.SignIn(context.Background(), &rpcParams)
	if err != nil {
		return Session{}, err
	}
	return Session{
		ID:    session.Id,
		Token: session.Token,
	}, nil
}
