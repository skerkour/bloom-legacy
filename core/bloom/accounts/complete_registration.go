package accounts

/*
pub fn registration_complete(input: auth::GuiRegistrationComplete) -> Result<Message, BloomError> {
    let auth_key = derive_auth_key(&input.username, &input.password)?;

    let message: Message = auth::RegistrationComplete {
        id: input.id,
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

func CompleteRegistration(params CompleteRegistrationParams) (Session, error) {
	return Session{
		ID:    "myRandomID",
		Token: "myRandomToken",
	}, nil
}
