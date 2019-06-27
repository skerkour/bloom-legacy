use crate::domain::CalendarEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

impl From<CalendarEvent> for EventResponse {
    fn from(event: CalendarEvent) -> Self {
        EventResponse {
            id: event.id,
            created_at: event.created_at,
            updated_at: event.updated_at,
            title: event.title,
            description: event.description,
            start_at: event.start_at,
            end_at: event.end_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateEventBody {
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEventBody {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}
