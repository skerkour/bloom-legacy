use super::{from_message, Message};
use bloom_models::contacts;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiListContacts {}
from_message!(GuiListContacts, Message::ContactsGuiListContacts);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiContacts {
    pub contacts: Vec<contacts::db::Conatct>,
}
from_message!(GuiContacts, Message::ContactsGuiContacts);
