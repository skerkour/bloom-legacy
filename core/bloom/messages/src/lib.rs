use serde::{Deserialize, Serialize};

pub mod kernel;
pub mod to_remove;

// services
pub mod auth;
pub mod calculator;
pub mod calendar;
pub mod notes;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    // kernel
    #[serde(rename = "empty")]
    Empty(kernel::Empty),
    #[serde(rename = "hello_world")]
    HelloWorld(kernel::HelloWorld),
    #[serde(rename = "error")]
    Error(kernel::Error),

    // auth
    #[serde(rename = "auth.registration_start")]
    AuthRegistrationStart(auth::RegistrationStart),
    #[serde(rename = "auth.registration_started")]
    AuthRegistrationStarted(auth::RegistrationStarted),
    #[serde(rename = "auth.registration_verify")]
    AuthRegistrationVerify(auth::RegistrationVerify),
    #[serde(rename = "auth.registration_complete")]
    AuthRegistrationComplete(auth::RegistrationComplete),
    #[serde(rename = "auth.registration_new_code")]
    AuthRegistrationNewCode(auth::RegistrationSendNewCode),
    #[serde(rename = "auth.session_started")]
    AuthSessionStarted(auth::Session),
    #[serde(rename = "auth.sign_in")]
    AuthSignIn(auth::SignIn),
    #[serde(rename = "auth.sign_out")]
    AuthSignOut(auth::SignOut),
    #[serde(rename = "auth.revoke_session")]
    AuthRevokeSession(auth::RevokeSesison),
    // auth gui
    #[serde(rename = "auth.gui.registration_complete")]
    AuthGuiRegistrationComplete(auth::GuiRegistrationComplete),
    #[serde(rename = "auth.gui.sign_in")]
    AuthGuiSignIn(auth::GuiSignIn),

    // notes gui
    #[serde(rename = "notes.gui.list_notes")]
    NotesGuiListNotes(notes::GuiListNotes),
    #[serde(rename = "notes.gui.notes")]
    NotesGuiNotes(notes::GuiNotes),
    #[serde(rename = "notes.gui.note")]
    NotesGuiNote(notes::GuiNote),
    #[serde(rename = "notes.gui.delete_note")]
    NotesGuiDeleteNote(notes::GuiDeleteNote),
    #[serde(rename = "notes.gui.get_archive")]
    NotesGuiGetArchive(notes::GuiGetArchive),
    #[serde(rename = "notes.gui.create_note")]
    NotesGuiCreateNote(notes::GuiCreateNote),
    #[serde(rename = "notes.gui.update_note")]
    NotesGuiUpdateNote(notes::GuiUpdateNote),

    // calculator gui
    #[serde(rename = "calculator.gui.expression")]
    CalculatorGuiExpression(calculator::GuiExpression),
    #[serde(rename = "calculator.gui.result")]
    CalculatorGuiResult(calculator::GuiResult),

    // calendar gui
    #[serde(rename = "calendar.gui.list_events")]
    CalendarGuiListEvents(calendar::GuiListEvents),
    #[serde(rename = "calendar.gui.events")]
    CalendarGuiEvents(calendar::GuiEvents),

    #[serde(rename = "gui.to_remove.tick")]
    ToRemoveTick(to_remove::Tick),
}

/// A macro to make it easy to create From impls for Message
#[macro_export]
macro_rules! from_message {
    ($t:ty, $m:expr) => {
        impl From<$t> for Message {
            fn from(data: $t) -> Message {
                $m(data)
            }
        }
    };
}
