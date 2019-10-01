use serde::{Deserialize, Serialize};

pub use auth;
pub use messages;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NativeMessage {
    pub id: Option<String>,
    pub message: messages::Message,
}

pub fn handle_message(message: messages::Message) -> messages::Message {
    let res = match message {
        messages::Message::AuthGuiRegistrationStart(message) => auth::registration_start(message),
        messages::Message::AuthRegistrationVerify(message) => auth::registration_verify(message),
        messages::Message::AuthRegistrationComplete(message) => {
            auth::registration_complete(message)
        }
        // TODO: handle message not implemented
        _ => {
            println!("unknown message: {:?}. sending back", &message);
            return messages::kernel::Error {
                code: "UNKNOWN_MESSAGE_TYPE".to_string(),
                message: "Unknown message type".to_string(),
            }
            .into();
        }
    };

    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
