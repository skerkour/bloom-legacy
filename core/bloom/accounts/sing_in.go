package accounts

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
	return Session{
		ID:    "myRandomID",
		Token: "myRandomTOken",
	}, nil
}
