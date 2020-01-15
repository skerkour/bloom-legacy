package auth

/*
pub fn registration_verify(message: auth::RegistrationVerify) -> Result<Message, BloomError> {
    let message: Message = message.into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}
*/

func VerifyRegistration(params VerifyRegistrationParams) ([]byte, error) {
	return nil, nil
}
