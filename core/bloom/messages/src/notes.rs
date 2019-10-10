use super::{from_message, Message};
use bloom_models::notes;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiListNotes {}
from_message!(GuiListNotes, Message::NotesGuiListNotes);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiNotes {
    pub notes: Vec<notes::db::Note>,
}
from_message!(GuiNotes, Message::NotesGuiNotes);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiDeleteNote {
    pub id: String,
}
from_message!(GuiDeleteNote, Message::NoteGuiDeleteNote);
