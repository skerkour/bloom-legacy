package accounts

/*
/// See https://bloom.sh/the-guide/projects/bloom/security/authentication.html#registration for the spec
pub fn registration_start(input: auth::RegistrationStart) -> Result<Message, BloomError> {
    let message: Message = input.into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}
*/

func StartRegistration(params StartRegistrationParams) ([]byte, error) {
	return nil, nil
}
