use super::{from_message, Message};
use bloom_models::calendar;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiListEvents {
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}
from_message!(GuiListEvents, Message::CalendarGuiListEvents);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiEvents {
    pub events: Vec<calendar::db::Event>,
}
from_message!(GuiEvents, Message::CalendarGuiEvents);
