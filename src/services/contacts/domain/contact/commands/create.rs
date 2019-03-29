use crate::{
    services::contacts::domain,
    services::common::events::EventMetadata,
    services::notes::validators,
    KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug)]
pub struct Create {
    pub addresses: Vec<domain::Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<domain::Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<domain::Organization>,
    pub phones: Vec<domain::Phone>,
    pub websites: Vec<domain::Website>,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for Create {
    type Aggregate = domain::Contact;
    type Event = domain::Contact;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();
        let data = domain::EventData::CreatedV1(domain::CreatedV1{
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

        return  Ok((domain::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: id,
            metadata: self.metaself.clone(),
        }, ()));
    }
}
