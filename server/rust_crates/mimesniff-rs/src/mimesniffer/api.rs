use url::Url;
use mime::Mime;

use super::magic::{is_unknown_mime_type, sniff_mime_type, sniff_mime_type_from_local_data};

/// Extension methods for MIME type sniffer
pub trait MimeTypeSniffer {
    /// sniff content for MIME type
    fn sniff_mime_type(&self) -> Option<&str>;
}

/// Extension methods for MIME type sniffer
pub trait MimeTypeSnifferExt: MimeTypeSniffer {
    /// sniff content for MIME type
    fn sniff_mime_type_ext(&self) -> Option<Mime> {
        self.sniff_mime_type()
            .and_then(|mime_type| mime_type.parse().ok())
    }
}

impl<T: MimeTypeSniffer> MimeTypeSnifferExt for T {}

/// Should we sniff content for MIME type
pub trait MimeTypeSniffable {
    /// should we sniff content
    fn should_sniff_mime_type(&self) -> bool;
}

impl<T: AsRef<[u8]>> MimeTypeSniffer for T {
    fn sniff_mime_type(&self) -> Option<&str> {
        sniff_mime_type_from_local_data(self.as_ref())
    }
}

impl<T: AsRef<[u8]>> MimeTypeSniffable for T {
    fn should_sniff_mime_type(&self) -> bool {
        true
    }
}

/// HTTP request with content, URL and MIME type hint.
pub struct HttpRequest<'a, T: 'a + AsRef<[u8]>, U: 'a + AsRef<str>> {
    pub content: &'a T,
    pub url: &'a U,
    pub type_hint: &'a str,
}

impl<'a, T: 'a + AsRef<[u8]>, U: 'a + AsRef<str>> MimeTypeSniffer for HttpRequest<'a, T, U> {
    fn sniff_mime_type(&self) -> Option<&str> {
        sniff_mime_type(self.content.as_ref(), self.url.as_ref(), self.type_hint)
    }
}

const SNIFFABLE_TYPES: &'static [&'static str] = &[
    // Many web servers are misconfigured to send text/plain for many
    // different types of content.
    "text/plain",
    // We want to sniff application/octet-stream for
    // application/x-chrome-extension, but nothing else.
    "application/octet-stream",
    // XHTML and Atom/RSS feeds are often served as plain xml instead of
    // their more specific mime types.
    "text/xml",
    "application/xml",
    // Check for false Microsoft Office MIME types.
    "application/msword",
    "application/vnd.ms-excel",
    "application/vnd.ms-powerpoint",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "application/vnd.ms-excel.sheet.macroenabled.12",
    "application/vnd.ms-word.document.macroenabled.12",
    "application/vnd.ms-powerpoint.presentation.macroenabled.12",
    "application/mspowerpoint",
    "application/msexcel",
    "application/vnd.ms-word",
    "application/vnd.ms-word.document.12",
    "application/vnd.msword",
];

impl<'a, T: 'a + AsRef<[u8]>> MimeTypeSniffable for HttpRequest<'a, T, Url> {
    fn should_sniff_mime_type(&self) -> bool {
        match self.url.scheme() {
            "" | "http" | "https" | "ftp" | "content" | "file" => {
                SNIFFABLE_TYPES
                    .iter()
                    .any(|&mime_type| mime_type == self.type_hint)
                    || is_unknown_mime_type(self.type_hint)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use super::*;

    #[test]
    fn test_mime_type_sniffer() {
        assert_eq!(b"%PDF-1.5".sniff_mime_type(), Some("application/pdf"));
    }

    #[test]
    fn test_request_sniffer() {
        let url = Url::parse("http://localhost/notes.ppt").unwrap();
        let req = HttpRequest {
            content: b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1",
            url: &url,
            type_hint: "text/plain",
        };

        assert!(req.should_sniff_mime_type());
        assert_eq!(req.sniff_mime_type(), Some("application/vnd.ms-powerpoint"));
        assert_eq!(
            req.sniff_mime_type_ext().unwrap(),
            "application/vnd.ms-powerpoint".parse::<Mime>().unwrap()
        );
    }
}
