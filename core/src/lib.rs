use serde::{Deserialize, Serialize};

pub use bloom_auth as auth;
pub use bloom_error as error;
pub use bloom_messages as messages;
pub use bloom_notes as notes;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativeMessage {
    pub id: Option<String>,
    pub message: messages::Message,
}

pub fn handle_message(message: messages::Message) -> messages::Message {
    let res = match message {
        // auth
        messages::Message::AuthRegistrationStart(message) => auth::registration_start(message),
        messages::Message::AuthRegistrationVerify(message) => auth::registration_verify(message),
        messages::Message::AuthGuiRegistrationComplete(message) => {
            auth::registration_complete(message)
        }
        messages::Message::AuthGuiSignIn(message) => auth::sign_in(message),
        messages::Message::AuthSignOut(_) => auth::sign_out(),

        // notes
        messages::Message::NotesGuiListNotes(_) => notes::list_notes(),

        // fallback
        _ => {
            println!("unknown message: {:?}. sending back", &message);
            return messages::kernel::Error {
                code: "UNKNOWN_MESSAGE_TYPE".to_string(),
                message: "Unknown message type".to_string(),
            }
            .into();
        }
    };

    match res {
        Ok(message) => message,
        Err(err) => {
            let err: messages::kernel::Error = err.into();
            err.into()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
