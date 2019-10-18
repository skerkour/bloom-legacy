use serde::{Deserialize, Serialize};

pub use bloom_auth as auth;
pub use bloom_calculator as calculator;
pub use bloom_calendar as calendar;
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
        messages::Message::AuthRegistrationStart(msg) => auth::registration_start(msg),
        messages::Message::AuthRegistrationVerify(msg) => auth::registration_verify(msg),
        messages::Message::AuthGuiRegistrationComplete(msg) => auth::registration_complete(msg),
        messages::Message::AuthGuiSignIn(msg) => auth::sign_in(msg),
        messages::Message::AuthSignOut(msg) => auth::sign_out(msg),

        // notes
        messages::Message::NotesGuiListNotes(msg) => notes::list_notes(msg),
        messages::Message::NotesGuiDeleteNote(msg) => notes::delete_note(msg),
        messages::Message::NotesGuiGetArchive(msg) => notes::get_archive(msg),
        messages::Message::NotesGuiCreateNote(msg) => notes::create_note(msg),
        messages::Message::NotesGuiUpdateNote(msg) => notes::update_note(msg),

        // calculator
        messages::Message::CalculatorGuiExpression(msg) => calculator::evaluate(msg),

        // calendar
        messages::Message::CalendarGuiListEvents(msg) => calendar::list_events(msg),

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
