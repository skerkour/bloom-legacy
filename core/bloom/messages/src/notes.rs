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
from_message!(GuiDeleteNote, Message::NotesGuiDeleteNote);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiGetArchive {}
from_message!(GuiGetArchive, Message::NotesGuiGetArchive);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiCreateNote {
    pub title: String,
    pub body: String,
    pub color: i64,
}
from_message!(GuiCreateNote, Message::NotesGuiCreateNote);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiNote {
    pub note: notes::db::Note,
}
from_message!(GuiNote, Message::NotesGuiNote);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiUpdateNote {
    pub note: notes::db::Note,
}
from_message!(GuiUpdateNote, Message::NotesGuiUpdateNote);
