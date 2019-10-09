use bloom_const::{
    auth::{KDF_AUTH_CONTEXT, KDF_AUTH_ID, KDF_AUTH_KEYLEN, KDF_PW_KEYLEN},
    API_URL,
};
use bloom_error::BloomError;
use bloom_messages::{auth, Message};
use crypto42::kdf::{argon2id, blake2b};

/// See https://theguide.bloom.sh/projects/bloom/security/authentication.html#registration for the spec
pub fn registration_start(input: auth::RegistrationStart) -> Result<Message, BloomError> {
    let message: Message = input.into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}

pub fn registration_verify(message: auth::RegistrationVerify) -> Result<Message, BloomError> {
    let message: Message = message.into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}

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

fn username_to_salt(username: &str) -> argon2id::Salt {
    let padded_username = pad_or_cut_str(username, argon2id::SALTBYTES);

    let mut salt = argon2id::Salt([0; argon2id::SALTBYTES]);
    (salt.0).clone_from_slice(padded_username.as_bytes());
    salt
}

fn pad_or_cut_str(s: &str, size: usize) -> String {
    let len = s.len();

    if len == size {
        return s.to_owned();
    } else if len < size {
        let mut out = String::new();
        out.push_str(s);
        for _ in 0..size - len {
            out.push(0x0 as char);
        }
        return out;
    } else {
        return s[..size].to_owned();
    }
}

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

pub fn sign_out() -> Result<Message, BloomError> {
    let message: Message = auth::SignOut {}.into();

    let client = reqwest::Client::new();
    let mut api_res = client.post(API_URL).json(&message).send()?;

    let ret: Message = api_res.json()?;

    return Ok(ret);
}

fn derive_auth_key(username: &str, password: &str) -> Result<Vec<u8>, BloomError> {
    let client_salt = username_to_salt(username);

    let pw_key = argon2id::derive_from_password(
        KDF_PW_KEYLEN,
        password.as_bytes(),
        &client_salt,
        argon2id::OPSLIMIT_INTERACTIVE,
        argon2id::MEMLIMIT_INTERACTIVE,
    )?;

    let auth_key = blake2b::derive_from_key(
        KDF_AUTH_KEYLEN,
        KDF_AUTH_ID,
        KDF_AUTH_CONTEXT,
        &pw_key.as_slice().into(),
    )?;

    return Ok(auth_key);
}

#[cfg(test)]
mod test {
    #[test]
    fn pad_or_cut_str() {
        assert_eq!("", super::pad_or_cut_str("hello world", 0));
        assert_eq!("hello", super::pad_or_cut_str("hello world", 5));
        assert_eq!("hello world", super::pad_or_cut_str("hello world", 11));
        assert_eq!(
            "hello world\x00\x00",
            super::pad_or_cut_str("hello world", 13)
        );
    }
}
