use crate::domain::contact;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{events::EventMetadata, KernelError};

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
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = contact::Contact;
    type Event = contact::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

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
    ) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();
        let data = contact::EventData::CreatedV1(contact::CreatedV1 {
            id,
            addresses: self.addresses.clone(),
            birthday: self.birthday.clone(),
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

        return Ok((
            contact::Event {
                id: uuid::Uuid::new_v4(),
                timestamp: chrono::Utc::now(),
                data,
                aggregate_id: id,
                metadata: self.metadata.clone(),
            },
            (),
        ));
    }
}
