# Calendar


## Overview

The calendar service is a user friendly calendar solution.

An user should be able to save, view and delete his events


## Scenarios


### 1

Sylvain wants to save his dentist appointment. He goes to his Bloom calendar, Tap the `Create` button. A dialog opens, Sylvain add the hours of the appointment and click `save`.


### 2

Benjamin wants to see what he have planned on his calendar in 3 months. He goes to his Bloom calendar, navigate to 3 months later, and here he is able to see what events have been planned between the start and the end of the month. By clicking on one event, a dialog open and he is able to learn more details about the event.
On the dialog detailing an event, There is a button to `Delete` the event.

## Non goals for this version

create a complete calendar, with locations, invites, exports, imports... **It should be the minimum working calendar**.

## Models

### Event

tables: `calendar_events`, `calendar_events_events` (?)

attributs:
```rust
pub struct Event {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,

    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,

    pub owner_id: uuid::Uuid,
}
```

events:
```rust
pub struct CreatedV1 {
    pub title: String,
    pub description: String,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: chrono::DateTime<chrono::Utc>,
}

pub struct TitleUpdatedV1 {
    pub title: String,
}

pub struct DescriptionUpdatedV1 {
    pub title: String,
}

pub struct StartAtUpdatedV1 {
    pub start_at: chrono::DateTime<chrono::Utc>,
}

pub struct EndAtUpdatedV1 {
    pub end_at: chrono::DateTime<chrono::Utc>,
}

// DeletedV1
```



##### Validations

limit extreme dates for `start_at` and `end_at`.
limit too much time between `start_at` and `end_at`.

## Views

https://vuetifyjs.com/en/components/calendars#calendar

`/calendar` the principal view, with a calendar (we should be able to change the timeframe: week, month...), with a `Create` button to open a dialog to add an event.




**Warning: How to handle timezones between frontend and backend?**
