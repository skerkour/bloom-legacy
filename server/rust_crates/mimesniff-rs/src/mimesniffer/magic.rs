use std::ascii::AsciiExt;

pub enum Magic {
    Number(&'static str, &'static [u8]),
    Mask(&'static str, &'static [u8], &'static [u8]),
    String(&'static str, &'static str),
}

impl Magic {
    pub fn mime_type(&self) -> &'static str {
        match *self {
            Magic::Number(mime_type, ..)
            | Magic::Mask(mime_type, ..)
            | Magic::String(mime_type, ..) => mime_type,
        }
    }

    pub fn matches(&self, content: &[u8]) -> bool {
        match *self {
            Magic::Number(_, ref magic) if content.len() >= magic.len() => {
                // Compare content header to a magic number where magic_entry can contain '.'
                // for single character of anything, allowing some bytes to be skipped.
                magic
                    .iter()
                    .zip(content[..magic.len()].iter())
                    .all(|(&m, &c)| m == b'.' || m == c)
            }
            Magic::Mask(_, magic, mask) if content.len() >= magic.len() => {
                // Like Magic::Number() except that it ANDs each byte with a mask before
                // the comparison, because there are some bits we don't care about.
                magic
                    .iter()
                    .zip(
                        content[..magic.len()]
                            .iter()
                            .zip(mask.iter())
                            .map(|(&c, &m)| c & m),
                    )
                    .all(|(&m, c)| m == b'.' || m == c)
            }
            Magic::String(_, ref magic) if content.len() >= magic.len() => magic
                .as_bytes()
                .eq_ignore_ascii_case(&content[..magic.len()]),
            _ => false,
        }
    }
}

const MAGIC_NUMBERS: &'static [Magic] = &[
    // Source: HTML 5 specification
    Magic::Number("application/pdf", b"%PDF-"),
    Magic::Number("application/postscript", b"%!PS-Adobe-"),
    Magic::Number("image/gif", b"GIF87a"),
    Magic::Number("image/gif", b"GIF89a"),
    Magic::Number("image/png", b"\x89PNG\x0D\x0A\x1A\x0A"),
    Magic::Number("image/jpeg", b"\xFF\xD8\xFF"),
    Magic::Number("image/bmp", b"BM"),
    // Source: Mozilla
    Magic::Number("text/plain", b"#!"), // Script
    Magic::Number("text/plain", b"%!"), // Script, similar to PS
    Magic::Number("text/plain", b"From"),
    Magic::Number("text/plain", b">From"),
    // Chrome specific
    Magic::Number("application/x-gzip", b"\x1F\x8B\x08"),
    Magic::Number("audio/x-pn-realaudio", b"\x2E\x52\x4D\x46"),
    Magic::Number(
        "video/x-ms-asf",
        b"\x30\x26\xB2\x75\x8E\x66\xCF\x11\xA6\xD9\x00\xAA\x00\x62\xCE\x6C",
    ),
    Magic::Number("image/tiff", b"I I"),
    Magic::Number("image/tiff", b"II*"),
    Magic::Number("image/tiff", b"MM\x00*"),
    Magic::Number("audio/mpeg", b"ID3"),
    Magic::Number("image/webp", b"RIFF....WEBPVP8 "),
    Magic::Number("video/webm", b"\x1A\x45\xDF\xA3"),
    Magic::Number("video/mpeg", b"\x00\x00\x01\x0B"),
    Magic::Number("audio/mpeg", b"\xFF\x0E"),
    Magic::Number("audio/mpeg", b"\xFF\x0F"),
    Magic::Number("application/zip", b"PK\x03\x04"),
    Magic::Number("application/x-rar-compressed", b"Rar!\x1A\x07\x00"),
    Magic::Number("application/x-msmetafile", b"\xD7\xCD\xC6\x9A"),
    Magic::Number("application/octet-stream", b"MZ"),
      // Sniffing for Flash
      //
      // Including these magic number for Flash is a trade off.
      //
      // Pros:
      //   * Flash is an important and popular file format
      //
      // Cons:
      //   * These patterns are fairly weak
      //   * If we mistakenly decide something is Flash, we will execute it
      //     in the origin of an unsuspecting site.  This could be a security
      //     vulnerability if the site allows users to upload content.
      //
      // On balance, we do not include these patterns.
      //
      // Magic::Number("application/x-shockwave-flash", b"CWS"),
      // Magic::Number("application/x-shockwave-flash", b"FLV"),
      // Magic::Number("application/x-shockwave-flash", b"FWS"),
];

// The number of content bytes we need to use all our magic numbers.  Feel free
// to increase this number if you add a longer magic number.
const BYTES_REQUIRED_FOR_MAGIC: usize = 42;

const MAX_BYTES_TO_SNIFF_HTML: usize = 512;
const MAX_BYTES_TO_SNIFF_BIN: usize = 512;
const MAX_BYTES_TO_SNIFF_XML: usize = 300;

const EXTRA_MAGIC_NUMBERS: &'static [Magic] = &[
    Magic::Number("image/x-xbitmap", b"#define"),
    Magic::Number("image/x-icon", b"\x00\x00\x01\x00"),
    Magic::Number("image/svg+xml", b"<?xml_version="),
    Magic::Number("audio/wav", b"RIFF....WAVEfmt "),
    Magic::Number("video/avi", b"RIFF....AVI LIST"),
    Magic::Number("audio/ogg", b"OggS"),
    Magic::Mask("video/mpeg", b"\x00\x00\x01\xB0", b"\xFF\xFF\xFF\xF0"),
    Magic::Mask("audio/mpeg", b"\xFF\xE0", b"\xFF\xE0"),
    Magic::Number("video/3gpp", b"....ftyp3g"),
    Magic::Number("video/3gpp", b"....ftypavcl"),
    Magic::Number("video/mp4", b"....ftyp"),
    Magic::Number("video/quicktime", b"....moov"),
    Magic::Number("application/x-shockwave-flash", b"CWS"),
    Magic::Number("application/x-shockwave-flash", b"FWS"),
    Magic::Number("video/x-flv", b"FLV"),
    Magic::Number("audio/x-flac", b"fLaC"),
    // RAW image types.
    Magic::Number("image/x-canon-cr2", b"II\x2a\x00\x10\x00\x00\x00CR"),
    Magic::Number("image/x-canon-crw", b"II\x1a\x00\x00\x00HEAPCCDR"),
    Magic::Number("image/x-minolta-mrw", b"\x00MRM"),
    Magic::Number("image/x-olympus-orf", b"MMOR"), // big-endian
    Magic::Number("image/x-olympus-orf", b"IIRO"), // little-endian
    Magic::Number("image/x-olympus-orf", b"IIRS"), // little-endian
    Magic::Number("image/x-fuji-raf", b"FUJIFILMCCD-RAW "),
    Magic::Number("image/x-panasonic-raw", b"IIU\x00\x08\x00\x00\x00"), // Panasonic .raw
    Magic::Number("image/x-panasonic-raw", b"IIU\x00\x18\x00\x00\x00"), // Panasonic .rw2
    Magic::Number("image/x-phaseone-raw", b"MMMMRaw"),
    Magic::Number("image/x-x3f", b"FOVb"),
];

pub fn sniff_mime_type<'a>(buf: &'a [u8], url: &'a str, type_hint: &'a str) -> Option<&'a str> {
    // By default, we assume we have enough content.
    // Each sniff routine may unset this if it wasn't provided enough content.
    let mut have_enough_content = true;

    // If the file has a Microsoft Office MIME type, we should only check that it
    // is a valid Office file.  Because this is the only reason we sniff files
    // with a Microsoft Office MIME type, we can return early.
    if is_office_mime_type(type_hint) {
        return sniff_for_invalid_office_docs(buf, type_hint);
    }

    // Cache information about the type_hint
    let hint_is_unknown_mime_type = is_unknown_mime_type(type_hint);

    // First check for HTML
    if hint_is_unknown_mime_type {
        // We're only willing to sniff HTML if the server has not supplied a mime
        // type, or if the type it did supply indicates that it doesn't know what
        // the type should be.
        let (matched, enough_content) = sniff_for_html(buf);

        if matched.is_some() {
            return matched; // We succeeded in sniffing HTML.  No more content needed.
        }

        have_enough_content &= enough_content;
    }

    // We're only willing to sniff for binary in 3 cases:
    // 1. The server has not supplied a mime type.
    // 2. The type it did supply indicates that it doesn't know what the type
    //    should be.
    // 3. The type is "text/plain" which is the default on some web servers and
    //    could be indicative of a mis-configuration that we shield the user from.
    let hint_is_text_plain = type_hint == "text/plain";

    if hint_is_unknown_mime_type || hint_is_text_plain {
        let (matched, enough_content) = sniff_binary(buf);

        have_enough_content &= enough_content;

        // If the server said the content was text/plain and it doesn't appear
        // to be binary, then we trust it.
        if matched.is_none() && hint_is_text_plain {
            return if have_enough_content {
                Some(type_hint)
            } else {
                None
            };
        }
    }

    // If we have plain XML, sniff XML subtypes.
    if type_hint == "text/xml" || type_hint == "application/xml" {
        // We're not interested in sniffing these types for images and the like.
        // Instead, we're looking explicitly for a feed.  If we don't find one
        // we're done and return early.
        let (matched, enough_content) = sniff_xml(buf, type_hint);

        have_enough_content &= enough_content;

        return if matched.is_some() {
            matched
        } else if have_enough_content {
            Some(type_hint)
        } else {
            None
        };
    }

    // Check the file extension and magic numbers to see if this is an Office
    // document.  This needs to be checked before the general magic numbers
    // because zip files and Office documents (OOXML) have the same magic number.
    let (matched, enough_content) = sniff_for_office_docs(buf, url);

    if matched.is_some() {
        return matched;
    }

    have_enough_content &= enough_content;

    // We're not interested in sniffing for magic numbers when the type_hint
    // is application/octet-stream.  Time to bail out.
    if type_hint == "application/octet-stream" {
        return Some(type_hint);
    }

    // Now we look in our large table of magic numbers to see if we can find
    // anything that matches the content.
    let (matched, enough_content) = sniff_for_magic_numbers(buf);

    if matched.is_some() {
        // We've matched a magic number.  No more content needed.
        return matched;
    }

    have_enough_content &= enough_content;

    if have_enough_content {
        Some(type_hint)
    } else {
        None
    }
}

pub fn sniff_mime_type_from_local_data<'a>(buf: &'a [u8]) -> Option<&str> {
    // First check the extra table.
    EXTRA_MAGIC_NUMBERS
        .iter()
        .find(|magic| magic.matches(buf))
        .map(|magic| magic.mime_type())
        .or_else(|| {
            // Finally check the original table.
            MAGIC_NUMBERS
                .iter()
                .find(|magic| magic.matches(buf))
                .map(|magic| magic.mime_type())
        })
}

const UNKNOWN_MIME_TYPES: &'static [&'static str] =
    &["", "unknown/unknown", "application/unknown", "*/*"];

pub fn is_unknown_mime_type(type_hint: &str) -> bool {
    UNKNOWN_MIME_TYPES
        .iter()
        .any(|&mime_type| mime_type == type_hint)
        || type_hint.bytes().find(|&c| c == b'/').is_none()
}

// Truncates |content size| to |max_size| and returns true if |content size| is at least |max_size|.
pub fn truncate_size(content: &[u8], max_size: usize) -> (&[u8], bool) {
    if content.len() >= max_size {
        (&content[..max_size], true)
    } else {
        (content, false)
    }
}

// Our HTML sniffer differs slightly from Mozilla.  For example, Mozilla will
// decide that a document that begins "<!DOCTYPE SOAP-ENV:Envelope PUBLIC " is
// HTML, but we will not.
macro_rules! html_tag {
    ($tag:expr) => (Magic::String("text/html", concat!("<", $tag)))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const SNIFFABLE_TAGS: &'static [Magic] = &[
    // XML processing directive.  Although this is not an HTML mime type, we sniff
   // for this in the HTML phase because text/xml is just as powerful as HTML and
   // we want to leverage our white space skipping technology.
   Magic::Number("text/xml", b"<?xml"), // Mozilla
   // DOCTYPEs
   html_tag!("!DOCTYPE html"), // HTML5 spec
   // Sniffable tags, ordered by how often they occur in sniffable documents.
   html_tag!("script"), // HTML5 spec, Mozilla
   html_tag!("html"), // HTML5 spec, Mozilla
   html_tag!("!--"),
   html_tag!("head"), // HTML5 spec, Mozilla
   html_tag!("iframe"), // Mozilla
   html_tag!("h1"), // Mozilla

   html_tag!("div"), // Mozilla
   html_tag!("font"), // Mozilla
   html_tag!("table"), // Mozilla
   html_tag!("a"), // Mozilla
   html_tag!("style"), // Mozilla
   html_tag!("title"), // Mozilla
   html_tag!("b"), // Mozilla
   html_tag!("body"), // Mozilla
   html_tag!("br"),
   html_tag!("p") /* Mozilla */
];

// Returns true and sets result if the content appears to be HTML.
// Clears have_enough_content if more data could possibly change the result.
pub fn sniff_for_html(content: &[u8]) -> (Option<&str>, bool) {
    // For HTML, we are willing to consider up to 512 bytes.
    // This may be overly conservative as IE only considers 256.
    let (buf, have_enough_content) = truncate_size(content, MAX_BYTES_TO_SNIFF_HTML);

    // We adopt a strategy similar to that used by Mozilla to sniff HTML tags,
    // but with some modifications to better match the HTML5 spec.
    (
        buf.iter()
            .position(|&c| !(c as char).is_whitespace())
            .and_then(|pos| {
                // |pos| now points to first non-whitespace character (or at end).
                let buf = &buf[pos..];

                SNIFFABLE_TAGS
                    .iter()
                    .find(|magic| magic.matches(buf))
                    .map(|magic| magic.mime_type())
            }),
        have_enough_content,
    )
}

const BYTE_ORDER_MARKS: &'static [Magic] = &[
    // UTF-16BE
    Magic::Number("text/plain", b"\xFE\xFF"),
    // UTF-16LE
    Magic::Number("text/plain", b"\xFF\xFE"),
    // UTF-8
    Magic::Number("text/plain", b"\xEF\xBB\xBF"),
];

// Whether a given byte looks like it might be part of binary content.
// Source: HTML5 spec
#[cfg_attr(rustfmt, rustfmt_skip)]
const BYTE_LOOKS_BINARY: &'static [u8] = &[
   1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1,  // 0x00 - 0x0F
   1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,  // 0x10 - 0x1F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x20 - 0x2F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x30 - 0x3F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x40 - 0x4F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x50 - 0x5F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x60 - 0x6F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x70 - 0x7F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x80 - 0x8F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0x90 - 0x9F
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0xA0 - 0xAF
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0xB0 - 0xBF
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0xC0 - 0xCF
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0xD0 - 0xDF
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0xE0 - 0xEF
   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  // 0xF0 - 0xFF
 ];

// Returns true and sets result to "application/octet-stream" if the content
// appears to be binary data. Otherwise, returns false and sets "text/plain".
// Clears have_enough_content if more data could possibly change the result.
pub fn sniff_binary(content: &[u8]) -> (Option<&str>, bool) {
    // There is no concensus about exactly how to sniff for binary content.
    // * IE 7: Don't sniff for binary looking bytes, but trust the file extension.
    // * Firefox 3.5: Sniff first 4096 bytes for a binary looking byte.
    // Here, we side with FF, but with a smaller buffer. This size was chosen
    // because it is small enough to comfortably fit into a single packet (after
    // allowing for headers) and yet large enough to account for binary formats
    // that have a significant amount of ASCII at the beginning (crbug.com/15314).
    let (buf, have_enough_content) = truncate_size(content, MAX_BYTES_TO_SNIFF_BIN);

    (
        if BYTE_ORDER_MARKS.iter().any(|magic| magic.matches(buf)) {
            // If there is BOM, we think the buffer is not binary.
            None
        } else if buf.iter().any(|&b| BYTE_LOOKS_BINARY[b as usize] != 0) {
            // Next we look to see if any of the bytes "look binary."
            // If we a see a binary-looking byte, we think the content is binary.
            Some("application/octet-stream")
        } else {
            // No evidence either way. Default to non-binary and, if truncated, clear
            // have_enough_content because there could be a binary looking byte in the
            // truncated data.
            None
        },
        have_enough_content,
    )
}

const XML_TAG: &'static [u8] = b"<?xml";
const DOCTYPE_TAG: &'static [u8] = b"<!DOCTYPE";

const MAGIC_XML: &'static [Magic] = &[
    Magic::String(
        "application/xhtml+xml",
        "<html xmlns=\"http://www.w3.\
         org/1999/xhtml\"",
    ),
    Magic::String("application/atom+xml", "<feed"),
    Magic::String("application/rss+xml", "<rss"),
];

// Returns true and sets result if the content appears to contain XHTML or a feed.
// Clears have_enough_content if more data could possibly change the result.
//
// TODO(evanm): this is similar but more conservative than what Safari does,
// while HTML5 has a different recommendation -- what should we do?
// TODO(evanm): this is incorrect for documents whose encoding isn't a superset
// of ASCII -- do we care?
pub fn sniff_xml<'a>(content: &[u8], type_hint: &'a str) -> (Option<&'a str>, bool) {
    let (mut buf, have_enough_content) = truncate_size(content, MAX_BYTES_TO_SNIFF_XML);

    for _ in 0..5 {
        match buf.iter().position(|&c| c == b'<') {
            Some(pos) => {
                if buf.len() >= pos + XML_TAG.len()
                    && XML_TAG.eq_ignore_ascii_case(&buf[pos..pos + XML_TAG.len()])
                {
                    buf = &buf[pos + XML_TAG.len()..];
                    continue;
                }

                if buf.len() >= pos + DOCTYPE_TAG.len()
                    && DOCTYPE_TAG.eq_ignore_ascii_case(&buf[pos..pos + DOCTYPE_TAG.len()])
                {
                    buf = &buf[pos + DOCTYPE_TAG.len()..];
                    continue;
                }

                return (
                    MAGIC_XML
                        .iter()
                        .find(|magic| magic.matches(&buf[pos..]))
                        .map(|magic| magic.mime_type())
                        .or(Some(type_hint)),
                    have_enough_content,
                );
            }
            _ => break,
        }
    }

    (None, have_enough_content)
}

pub enum OfficeDocType {
    Word,
    Excel,
    PowerPoint,
}

const OFFICE_EXTENSION_TYPES: &'static [(OfficeDocType, &'static str)] = &[
    (OfficeDocType::Word, ".doc"),
    (OfficeDocType::Excel, ".xls"),
    (OfficeDocType::PowerPoint, ".ppt"),
    (OfficeDocType::Word, ".docx"),
    (OfficeDocType::Excel, ".xlsx"),
    (OfficeDocType::PowerPoint, ".pptx"),
];

// The number of content bytes we need to use all our Microsoft Office magic numbers.
const BYTES_REQUIRED_FOR_OFFICE_MAGIC: usize = 8;

const OFFICE_MAGIC_NUMBERS: &'static [Magic] = &[
    Magic::Number("CFB", b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1"),
    Magic::Number("OOXML", b"PK\x03\x04"),
];

pub fn is_office_mime_type(type_hint: &str) -> bool {
    match type_hint {
        "application/msword"
        | "application/vnd.ms-excel"
        | "application/vnd.ms-powerpoint"
        | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
        | "application/vnd.ms-excel.sheet.macroenabled.12"
        | "application/vnd.ms-word.document.macroenabled.12"
        | "application/vnd.ms-powerpoint.presentation.macroenabled.12"
        | "application/mspowerpoint"
        | "application/msexcel"
        | "application/vnd.ms-word"
        | "application/vnd.ms-word.document.12"
        | "application/vnd.msword" => true,
        _ => false,
    }
}

// This function checks for files that have a Microsoft Office MIME type
// set, but are not actually Office files.
//
// If this is not actually an Office file, |*result| is set to
// "application/octet-stream", otherwise it is not modified.
//
// Returns false if additional data is required to determine the file type, or
// true if there is enough data to make a decision.
pub fn sniff_for_invalid_office_docs<'a>(content: &[u8], type_hint: &'a str) -> Option<&'a str> {
    match truncate_size(content, BYTES_REQUIRED_FOR_OFFICE_MAGIC) {
        (buf, true) => {
            // Check our table of magic numbers for Office file types.  If it does not
            // match one, the MIME type was invalid.  Set it instead to a safe value.
            Some(
                OFFICE_MAGIC_NUMBERS
                    .iter()
                    .find(|magic| magic.matches(buf))
                    .map_or_else(|| "application/octet-stream", |_| type_hint),
            )
        }
        _ => None,
    }
}

// Returns true and sets result if the content matches any of
// kOfficeMagicNumbers, and the URL has the proper extension.
// Clears |have_enough_content| if more data could possibly change the result.
pub fn sniff_for_office_docs<'a>(content: &[u8], url: &'a str) -> (Option<&'a str>, bool) {
    let (buf, have_enough_content) = truncate_size(content, BYTES_REQUIRED_FOR_OFFICE_MAGIC);

    (
        OFFICE_MAGIC_NUMBERS
            .iter()
            .find(|magic| magic.matches(buf))
            .and_then(|magic| {
                OFFICE_EXTENSION_TYPES
                    .iter()
                    .filter(|&&(_, ref file_ext)| file_ext.len() <= url.len())
                    .find(|&&(_, ref file_ext)| {
                        file_ext.eq_ignore_ascii_case(&url[url.len() - file_ext.len()..])
                    })
                    .and_then(|&(ref doc_type, _)| match magic.mime_type() {
                        "CFB" => match *doc_type {
                            OfficeDocType::Word => Some("application/msword"),
                            OfficeDocType::Excel => Some("application/vnd.ms-excel"),
                            OfficeDocType::PowerPoint => Some("application/vnd.ms-powerpoint"),
                        },
                        "OOXML" => match *doc_type {
                            OfficeDocType::Word => Some(
                                "application/vnd.openxmlformats-officedocument.\
                                 wordprocessingml.document",
                            ),
                            OfficeDocType::Excel => Some(
                                "application/vnd.openxmlformats-officedocument.spreadsheetml.\
                                 sheet",
                            ),
                            OfficeDocType::PowerPoint => Some(
                                "application/vnd.openxmlformats-officedocument.\
                                 presentationml.presentation",
                            ),
                        },
                        _ => None,
                    })
            }),
        have_enough_content,
    )
}

// Returns true and sets result if the content matches any of kMagicNumbers.
// Clears have_enough_content if more data could possibly change the result.
pub fn sniff_for_magic_numbers(content: &[u8]) -> (Option<&str>, bool) {
    let (buf, have_enough_content) = truncate_size(content, BYTES_REQUIRED_FOR_MAGIC);

    (
        MAGIC_NUMBERS
            .iter()
            .find(|magic| magic.matches(buf))
            .map(|magic| magic.mime_type()),
        have_enough_content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_number() {
        let m = Magic::Number("application/pdf", b"%PDF-");

        assert!(m.matches(b"%PDF-1.5"));
        assert!(!m.matches(b"test"));
        assert!(!m.matches(b"testtest"));
    }

    #[test]
    fn test_magic_mask() {
        let m = Magic::Mask("video/mpeg", b"\x00\x00\x01\xB0", b"\xFF\xFF\xFF\xF0");

        assert!(m.matches(b"\x00\x00\x01\xB3\x14\x00\xc8\x11"));
        assert!(!m.matches(b"\x14\x00\xc8\x11\x00\x00\x01\xB3"));
    }

    #[test]
    fn test_magic_string() {
        let m = Magic::String("text/html", "<body");

        assert!(m.matches(b"<body"));
        assert!(m.matches(b"<BODY"));
        assert!(!m.matches(b"<html"));
    }

    #[test]
    fn test_invalid_office_docs() {
        assert_eq!(
            None,
            sniff_for_invalid_office_docs(b"test", "application/msword")
        );
        assert_eq!(
            Some("application/msword"),
            sniff_for_invalid_office_docs(
                b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1\x00\x00",
                "application/msword"
            )
        );
        assert_eq!(
            Some("application/octet-stream"),
            sniff_for_invalid_office_docs(&[0_u8; 12], "application/msword")
        );
    }

    #[test]
    fn test_unknown_mime_type() {
        assert!(is_unknown_mime_type("unknown/unknown"));
        assert!(is_unknown_mime_type("unknown"));
        assert!(!is_unknown_mime_type("application/msword"));
    }

    #[test]
    fn test_sniff_for_html() {
        assert_eq!((None, false), sniff_for_html(b"test"));
        assert_eq!((Some("text/html"), false), sniff_for_html(b"<body"));

        let mut v = vec![];

        v.extend_from_slice(&[32_u8; 300][..]);
        v.extend_from_slice(b"<iframe");
        v.extend_from_slice(&[32_u8; 300][..]);

        assert_eq!((Some("text/html"), true), sniff_for_html(&v));

        v.clear();
        v.extend_from_slice(&[32_u8; 510][..]);
        v.extend_from_slice(b"<iframe");

        assert_eq!((None, true), sniff_for_html(&v));
    }

    #[test]
    fn test_sniff_binary() {
        assert_eq!((None, false), sniff_binary(b"\xEF\xBB\xBFtest"));
        assert_eq!(
            (Some("application/octet-stream"), false),
            sniff_binary(b"\x0F")
        );
        assert_eq!((None, true), sniff_binary(&[b'\n'; 522]));
        assert_eq!((None, false), sniff_binary(b"\n"));
    }

    #[test]
    fn test_sniff_xml() {
        assert_eq!(
            (Some("text/xml"), false),
            sniff_xml(b"<note><to>Tove</to></note>", "text/xml")
        );
        assert_eq!(
            (Some("text/xml"), false),
            sniff_xml(
                b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                                <note><to>Tove</to></note>",
                "text/xml"
            )
        );
        assert_eq!(
            (Some("text/xml"), false),
            sniff_xml(
                b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                                <!DOCTYPE note SYSTEM \"Note.dtd\">\
                                <note><to>Tove</to></note>",
                "text/xml"
            )
        );

        assert_eq!(
            (Some("application/rss+xml"), false),
            sniff_xml(
                b"<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\
                                <rss version=\"2.0\">",
                "text/xml"
            )
        );
    }

    #[test]
    fn test_sniff_for_office_docs() {
        assert_eq!(
            (None, false),
            sniff_for_office_docs(b"test", "http://note.docx")
        );
        assert_eq!(
            (Some("application/vnd.ms-powerpoint"), true),
            sniff_for_office_docs(b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1", "http://node.ppt")
        );
        assert_eq!(
            (
                Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
                true
            ),
            sniff_for_office_docs(b"PK\x03\x04\x00\x00\x00\x00", "http://note.docx")
        );
    }

    #[test]
    fn test_sniff_for_magic_numbers() {
        let mut v = vec![];

        v.extend_from_slice(b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46");

        assert_eq!((Some("image/jpeg"), false), sniff_for_magic_numbers(&v));

        v.extend_from_slice(&[b'\x00'; 40]);

        assert_eq!((Some("image/jpeg"), true), sniff_for_magic_numbers(&v));
    }

    #[test]
    fn test_sniff_mime_type_from_local_data() {
        let mut v = vec![];

        v.extend_from_slice(b"\x00\x00\x01\xB3\x14\x00\xc8\x11");
        v.extend_from_slice(&[b'\x00'; 40]);

        assert_eq!(Some("video/mpeg"), sniff_mime_type_from_local_data(&v));

        v.clear();
        v.extend_from_slice(b"\xFF\xD8\xFF\xE0\x00\x10\x4A\x46");
        v.extend_from_slice(&[b'\x00'; 40]);

        assert_eq!(Some("image/jpeg"), sniff_mime_type_from_local_data(&v));

         v.clear();
        v.extend_from_slice(b"\x89\x50\x4E\x47\x0D\x0A\x1A\x0A\xFF");
        v.extend_from_slice(&[b'\x00'; 40]);

        assert_eq!(Some("image/png"), sniff_mime_type_from_local_data(&v));
    }
}
