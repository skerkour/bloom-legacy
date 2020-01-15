package accounts

/*
pub fn sign_out(_: auth::SignOut) -> Result<Message, BloomError> {
    let message: Message = auth::SignOut {}.into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}
*/
func SignOut() error {
	return nil
}
