use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Create {
    pub addresses: Vec<contact::Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<contact::Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<contact::Organization>,
    pub phones: Vec<contact::Phone>,
    pub websites: Vec<contact::Website>,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Create {
    type Aggregate = contact::Contact;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Created {
            timestamp: chrono::Utc::now(),
            id: uuid::Uuid::new_v4(),
            addresses: self.addresses.clone(),
            birthday: self.birthday,
            company: self.company.clone(),
            emails: self.emails.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            notes: self.notes.clone(),
            occupation: self.occupation.clone(),
            organizations: self.organizations.clone(),
            phones: self.phones.clone(),
            websites: self.websites.clone(),
            owner_id: self.owner_id,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub addresses: Vec<contact::Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<contact::Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<contact::Organization>,
    pub phones: Vec<contact::Phone>,
    pub websites: Vec<contact::Website>,
    pub owner_id: uuid::Uuid,
}

impl Event for Created {
    type Aggregate = contact::Contact;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            addresses: self.addresses.clone(),
            birthday: self.birthday,
            company: self.company.clone(),
            emails: self.emails.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            notes: self.notes.clone(),
            occupation: self.occupation.clone(),
            organizations: self.organizations.clone(),
            phones: self.phones.clone(),
            websites: self.websites.clone(),
            owner_id: self.owner_id,
        };
    }
}
