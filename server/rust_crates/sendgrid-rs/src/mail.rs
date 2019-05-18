use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json;

use errors::{SendgridError, SendgridResult};

macro_rules! add_field {
    // Create a setter that appends.
    ($method:ident << $field:ident: $ty:ty) => {
        pub fn $method(mut self, data: $ty) -> Mail<'a> {
            self.$field.push(data);
            self
        }
    };

    // Create a setter that stores.
    ($method:ident = $field:ident: $ty:ty) => {
        pub fn $method(mut self, data: $ty) -> Mail<'a> {
            self.$field = data;
            self
        }
    };

    // Create a setter that inserts into a map.
    ($method:ident <- $field:ident: $ty:ty) => {
        pub fn $method(mut self, id: String, data: $ty) -> Mail<'a> {
            self.$field.insert(id, data);
            self
        }
    };
}

#[derive(Debug)]
pub struct Destination<'a> {
    pub address: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Default)]
/// This is a representation of a valid SendGrid message. It has support for
/// all of the fields in the V2 API.
pub struct Mail<'a> {
    pub to: Vec<Destination<'a>>,
    pub cc: Vec<&'a str>,
    pub bcc: Vec<&'a str>,
    pub from: &'a str,
    pub subject: &'a str,
    pub html: &'a str,
    pub text: &'a str,
    pub from_name: &'a str,
    pub reply_to: &'a str,
    pub date: &'a str,
    pub attachments: HashMap<String, String>,
    pub content: HashMap<String, &'a str>,
    pub headers: HashMap<String, &'a str>,
    pub x_smtpapi: &'a str,
}

impl<'a> Mail<'a> {
    /// Returns a new Mail struct to send with a client. All of the fields are
    /// initially empty.
    pub fn new() -> Mail<'a> {
        Mail::default()
    }

    /// Adds a CC recipient to the Mail struct.
    add_field!(add_cc << cc: &'a str);

    /// Adds a to recipient to the Mail struct.
    add_field!(add_to << to: Destination<'a>);

    /// Set the from address for the Mail struct. This can be changed, but there
    /// is only one from address per message.
    add_field!(add_from = from: &'a str);

    /// Set the subject of the message.
    add_field!(add_subject = subject: &'a str);

    /// This function sets the HTML content for the message.
    add_field!(add_html = html: &'a str);

    /// Set the text content of the message.
    add_field!(add_text = text: &'a str);

    /// Add a BCC address to the message.
    add_field!(add_bcc << bcc: &'a str);

    /// Set the from name for the message.
    add_field!(add_from_name = from_name: &'a str);

    /// Set the reply to address for the message.
    add_field!(add_reply_to = reply_to: &'a str);

    /// Set the date for the message. This must be a valid RFC 822 timestamp.
    // TODO(richo) Should this be a chronos::Utc ?
    add_field!(add_date = date: &'a str);

    /// Convenience method when using Mail as a builder.
    pub fn build(self) -> Mail<'a> {
        self
    }

    /// Add an attachment for the message. You can pass the name of a file as a
    /// path on the file system.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let message = Mail::new()
    ///     .add_attachment("/path/to/file/contents.txt");
    /// ```
    pub fn add_attachment<P: AsRef<Path>>(mut self, path: P) -> SendgridResult<Mail<'a>> {
        let mut file = File::open(&path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        if let Some(name) = path.as_ref().to_str() {
            self.attachments.insert(String::from(name), data);
        } else {
            return Err(SendgridError::InvalidFilename);
        }

        Ok(self)
    }

    /// Add content for inline images in the message.
    add_field!(add_content <- content: &'a str);

    /// Add a custom header for the message. These are usually prefixed with
    /// 'X' or 'x' per the RFC specifications.
    add_field!(add_header <- headers: &'a str);

    /// Used internally for string encoding. Not needed for message building.
    pub(crate) fn make_header_string(&mut self) -> SendgridResult<String> {
        let string = serde_json::to_string(&self.headers)?;
        Ok(string)
    }

    /// Add an X-SMTPAPI string to the message. This can be done by using the
    /// 'rustc_serialize' crate and JSON encoding a map or custom struct. Or
    /// a regular String type can be escaped and used.
    add_field!(add_x_smtpapi = x_smtpapi: &'a str);
}
