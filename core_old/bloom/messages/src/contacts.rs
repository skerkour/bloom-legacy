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
pub struct GuiContact {
    pub contact: contacts::db::Conatct,
}
from_message!(Box<GuiContact>, Message::ContactsGuiContact);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiContacts {
    pub contacts: Vec<contacts::db::Conatct>,
}
from_message!(GuiContacts, Message::ContactsGuiContacts);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiCreateContact {
    pub first_name: String,
    pub last_name: String,
    pub notes: String,
    pub addresses: serde_json::Value,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub organizations: serde_json::Value,
    pub emails: serde_json::Value,
    pub phones: serde_json::Value,
    pub websites: serde_json::Value,
    pub device_id: String,
}
from_message!(Box<GuiCreateContact>, Message::ContactsGuiCreateContact);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiDeleteContact {
    pub id: String,
}
from_message!(GuiDeleteContact, Message::ContactsGuiDeleteContact);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiUpdateContact {
    pub contact: contacts::db::Conatct,
}
from_message!(Box<GuiUpdateContact>, Message::ContactsGuiUpdateContact);
