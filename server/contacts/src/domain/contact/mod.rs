mod aggregate;
mod commands;

pub use aggregate::{
    Address, AddressLabel, Contact, Email, EmailLabel, Organization, Phone, PhoneLabel, Website,
    WebsiteLabel,
};
pub use commands::{
    create::{Create, Created},
    delete::{Delete, Deleted},
    update_addresses::{AddressesUpdated, UpdateAddresses},
    update_birthday::{BirthdayUpdated, UpdateBirthday},
    update_emails::{EmailsUpdated, UpdateEmails},
    update_first_name::{FirstNameUpdated, UpdateFirstName},
    update_last_name::{LastNameUpdated, UpdateLastName},
    update_notes::{NotesUpdated, UpdateNotes},
    update_organizations::{OrganizationsUpdated, UpdateOrganizations},
    update_phones::{PhonesUpdated, UpdatePhones},
    update_websites::{UpdateWebsites, WebsitesUpdated},
};
