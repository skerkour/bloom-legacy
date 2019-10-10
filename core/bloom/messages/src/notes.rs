use super::Message;
use bloom_models::notes;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiListNotes {}

impl From<GuiListNotes> for Message {
    fn from(data: GuiListNotes) -> Self {
        Message::NotesGuiListNotes(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiNotes {
    pub notes: Vec<notes::db::Note>,
}

impl From<GuiNotes> for Message {
    fn from(data: GuiNotes) -> Self {
        Message::NotesGuiNotes(data)
    }
}
