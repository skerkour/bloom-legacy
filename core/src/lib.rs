pub use auth;
pub use messages;

pub fn handle_message(message: messages::Message) -> messages::Message {
    println!("message received: {:?}", &message);
    match message {
        messages::Message::AuthGuiRegistrationStart(message) => auth::registration_start(message),
        messages::Message::AuthRegistrationVerify(message) => auth::registration_verify(message),
        messages::Message::AuthRegistrationComplete(message) => {
            auth::registration_complete(message)
        }
        // TODO: handle message not implemented
        _ => {
            println!("unknown message: {:?}. sending back", &message);
            return messages::kernel::Error {
                code: "UNKNOWN_MESSAGE".to_string(),
                message: "Unknown message type".to_string(),
            }
            .into();
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
