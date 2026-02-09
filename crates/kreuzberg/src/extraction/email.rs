//! Email extraction functions.
//!
//! Parses .eml (RFC822) and .msg (Outlook) email files using `mail-parser`.
//! Extracts message content, headers, and attachment information.
//!
//! # Features
//!
//! - **EML support**: RFC822 format parsing
//! - **HTML to text**: Strips HTML tags from HTML email bodies
//! - **Metadata extraction**: Sender, recipients, subject, message ID
//! - **Attachment list**: Names of all attachments (content not extracted)
//!
//! # Example
//!
//! ```rust,no_run
//! use kreuzberg::extraction::email::parse_eml_content;
//!
//! # fn example() -> kreuzberg::Result<()> {
//! let eml_bytes = std::fs::read("message.eml")?;
//! let result = parse_eml_content(&eml_bytes)?;
//!
//! println!("From: {:?}", result.from_email);
//! println!("Subject: {:?}", result.subject);
//! # Ok(())
//! # }
//! ```
use bytes::Bytes;

use crate::error::{KreuzbergError, Result};
use crate::types::{EmailAttachment, EmailExtractionResult};
use mail_parser::MimeHeaders;
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

static HTML_TAG_RE: OnceLock<Regex> = OnceLock::new();
static SCRIPT_RE: OnceLock<Regex> = OnceLock::new();
static STYLE_RE: OnceLock<Regex> = OnceLock::new();
static WHITESPACE_RE: OnceLock<Regex> = OnceLock::new();

fn html_tag_regex() -> &'static Regex {
    HTML_TAG_RE.get_or_init(|| Regex::new(r"<[^>]+>").unwrap())
}

fn script_regex() -> &'static Regex {
    SCRIPT_RE.get_or_init(|| Regex::new(r"(?i)<script[^>]*>.*?</script>").unwrap())
}

fn style_regex() -> &'static Regex {
    STYLE_RE.get_or_init(|| Regex::new(r"(?i)<style[^>]*>.*?</style>").unwrap())
}

fn whitespace_regex() -> &'static Regex {
    WHITESPACE_RE.get_or_init(|| Regex::new(r"\s+").unwrap())
}

/// Detect UTF-16 encoding (with or without BOM) and transcode to UTF-8 if needed.
///
/// `mail_parser` expects ASCII/UTF-8 input. If the EML file is encoded as
/// UTF-16, we transcode it to UTF-8 first.
///
/// Detection strategy:
/// 1. Check for BOM (`FF FE` = LE, `FE FF` = BE)
/// 2. If no BOM, use heuristic: EML files start with ASCII headers, so
///    alternating zero bytes indicate UTF-16 encoding.
fn maybe_transcode_utf16(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 4 {
        return None;
    }

    let (is_le, skip) = if data[0] == 0xFF && data[1] == 0xFE {
        (true, 2)
    } else if data[0] == 0xFE && data[1] == 0xFF {
        (false, 2)
    } else if data[1] == 0x00 && data[3] == 0x00 && data[0] != 0x00 && data[2] != 0x00 {
        // No BOM, but looks like UTF-16 LE (e.g. "M\0I\0M\0E\0")
        (true, 0)
    } else if data[0] == 0x00 && data[2] == 0x00 && data[1] != 0x00 && data[3] != 0x00 {
        // No BOM, but looks like UTF-16 BE (e.g. "\0M\0I\0M\0E")
        (false, 0)
    } else {
        return None;
    };

    let payload = &data[skip..];
    let even_len = payload.len() & !1;
    let u16_iter = (0..even_len).step_by(2).map(|i| {
        if is_le {
            u16::from_le_bytes([payload[i], payload[i + 1]])
        } else {
            u16::from_be_bytes([payload[i], payload[i + 1]])
        }
    });

    match String::from_utf16(&u16_iter.collect::<Vec<u16>>()) {
        Ok(s) => Some(s.into_bytes()),
        Err(_) => None,
    }
}

/// Parse .eml file content (RFC822 format)
pub fn parse_eml_content(data: &[u8]) -> Result<EmailExtractionResult> {
    // Transcode UTF-16 to UTF-8 if a BOM is detected
    let data = if let Some(transcoded) = maybe_transcode_utf16(data) {
        std::borrow::Cow::Owned(transcoded)
    } else {
        std::borrow::Cow::Borrowed(data)
    };

    let message = mail_parser::MessageParser::default()
        .parse(&data)
        .ok_or_else(|| KreuzbergError::parsing("Failed to parse EML file: invalid email format".to_string()))?;

    let subject = message.subject().map(|s| s.to_string());

    let from_email = message
        .from()
        .and_then(|from| from.first())
        .and_then(|addr| addr.address())
        .map(|s| s.to_string());

    let to_emails: Vec<String> = message
        .to()
        .map(|to| {
            to.iter()
                .filter_map(|addr| addr.address().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let cc_emails: Vec<String> = message
        .cc()
        .map(|cc| {
            cc.iter()
                .filter_map(|addr| addr.address().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let bcc_emails: Vec<String> = message
        .bcc()
        .map(|bcc| {
            bcc.iter()
                .filter_map(|addr| addr.address().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let date = message.date().map(|d| d.to_rfc3339());

    let message_id = message.message_id().map(|id| id.to_string());

    let plain_text = message.body_text(0).map(|s| s.to_string());

    let html_content = message.body_html(0).map(|s| s.to_string());

    let cleaned_text = if let Some(ref plain) = plain_text {
        plain.clone()
    } else if let Some(html) = &html_content {
        clean_html_content(html)
    } else {
        String::new()
    };

    let mut attachments = Vec::with_capacity(message.attachments().count().min(20));
    for attachment in message.attachments() {
        let filename = attachment.attachment_name().map(|s| s.to_string());

        let mime_type = attachment
            .content_type()
            .map(|ct| {
                let content_type_str = format!("{}/{}", ct.ctype(), ct.subtype().unwrap_or("octet-stream"));
                parse_content_type(&content_type_str)
            })
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let data = attachment.contents();
        let size = data.len();

        let is_image = is_image_mime_type(&mime_type);

        attachments.push(EmailAttachment {
            name: filename.clone(),
            filename,
            mime_type: Some(mime_type),
            size: Some(size),
            is_image,
            data: Some(Bytes::copy_from_slice(data)),
        });
    }

    let metadata = build_metadata(
        &subject,
        &from_email,
        &to_emails,
        &cc_emails,
        &bcc_emails,
        &date,
        &message_id,
        &attachments,
    );

    Ok(EmailExtractionResult {
        subject,
        from_email,
        to_emails,
        cc_emails,
        bcc_emails,
        date,
        message_id,
        plain_text,
        html_content,
        cleaned_text,
        attachments,
        metadata,
    })
}

/// Parse .msg file content (Outlook format).
///
/// Reads MSG files directly via the CFB (OLE Compound Document) format,
/// extracting text properties and attachment metadata without the overhead
/// of hex-encoding attachment binary data (which caused hangs on large files
/// with the previous `msg_parser` dependency).
///
/// Some MSG files have FAT headers declaring more sectors than the file
/// actually contains.  The strict `cfb` crate rejects these.  When that
/// happens we pad the data with zero bytes so the sector count matches
/// the FAT and retry – the real streams are still within the original
/// data range and parse correctly.
pub fn parse_msg_content(data: &[u8]) -> Result<EmailExtractionResult> {
    use std::borrow::Cow;
    use std::io::Cursor;

    // Try strict CFB parsing first; fall back to lenient padding.
    let padded: Cow<'_, [u8]>;
    let data_ref: &[u8] = match cfb::CompoundFile::open(Cursor::new(data)) {
        Ok(_) => data,
        Err(_first_err) => {
            padded = pad_cfb_to_fat_size(data);
            if std::ptr::eq(padded.as_ref(), data) {
                // Padding didn't help – propagate original error.
                return Err(KreuzbergError::parsing(format!(
                    "Failed to parse MSG file: {_first_err}"
                )));
            }
            &padded
        }
    };

    let mut comp = cfb::CompoundFile::open(Cursor::new(data_ref))
        .map_err(|e| KreuzbergError::parsing(format!("Failed to parse MSG file: {e}")))?;

    extract_msg_from_cfb(&mut comp)
}

/// Pad an OLE/CFB file so the sector count matches the FAT header.
///
/// Some MSG writers emit FAT tables that reference sectors beyond the
/// physical end of the file.  The `cfb` crate rightfully rejects these
/// as "Malformed FAT".  By zero-padding to the declared size we let cfb
/// open the file; streams within the original range parse normally while
/// the padded area is treated as free sectors.
fn pad_cfb_to_fat_size(data: &[u8]) -> std::borrow::Cow<'_, [u8]> {
    use std::borrow::Cow;

    // OLE header is at least 76 bytes; magic D0 CF 11 E0 A1 B1 1A E1.
    if data.len() < 76 || data[..4] != [0xD0, 0xCF, 0x11, 0xE0] {
        return Cow::Borrowed(data);
    }

    let sector_power = u16::from_le_bytes([data[30], data[31]]) as u32;
    if !(9..=16).contains(&sector_power) {
        return Cow::Borrowed(data);
    }
    let sector_size = 1u64 << sector_power;

    // Number of FAT sectors is at header offset 44 (LE u32).
    let fat_sectors = u32::from_le_bytes([data[44], data[45], data[46], data[47]]) as u64;
    // Each FAT sector holds sector_size/4 entries; each entry maps one sector.
    let fat_entries = fat_sectors * (sector_size / 4);
    // File must be at least: 1 header sector + fat_entries data sectors.
    let needed = (1 + fat_entries) * sector_size;

    // Cap at 256 MB to avoid pathological headers causing huge allocations.
    if needed > 256 * 1024 * 1024 || (data.len() as u64) >= needed {
        return Cow::Borrowed(data);
    }

    let mut padded = data.to_vec();
    padded.resize(needed as usize, 0);
    Cow::Owned(padded)
}

/// Internal: extract email fields from an already-opened CFB compound file.
fn extract_msg_from_cfb<F: std::io::Read + std::io::Seek>(
    comp: &mut cfb::CompoundFile<F>,
) -> Result<EmailExtractionResult> {
    // --- message-level properties ------------------------------------------

    let subject = read_msg_string_prop(comp, "", 0x0037); // PR_SUBJECT
    let sender_name = read_msg_string_prop(comp, "", 0x0C1A); // PR_SENDER_NAME
    let from_email = read_msg_string_prop(comp, "", 0x0C1F) // PR_SENDER_EMAIL_ADDRESS
        .or_else(|| read_msg_string_prop(comp, "", 0x0065)) // PR_SENT_REPRESENTING_EMAIL
        .filter(|s| !s.is_empty());
    let display_to = read_msg_string_prop(comp, "", 0x0E04); // PR_DISPLAY_TO
    let display_cc = read_msg_string_prop(comp, "", 0x0E03); // PR_DISPLAY_CC
    let display_bcc = read_msg_string_prop(comp, "", 0x0E02); // PR_DISPLAY_BCC
    let body = read_msg_string_prop(comp, "", 0x1000); // PR_BODY
    let html_body = read_msg_string_prop(comp, "", 0x1013); // PR_BODY_HTML
    let message_id = read_msg_string_prop(comp, "", 0x1035) // PR_INTERNET_MESSAGE_ID
        .filter(|s| !s.is_empty());
    let headers = read_msg_string_prop(comp, "", 0x007D); // PR_TRANSPORT_MESSAGE_HEADERS

    // Parse date from transport headers (e.g. "Date: Mon, 1 Jan 2024 …").
    let date = headers.as_ref().and_then(|h| {
        h.lines()
            .find(|line| line.starts_with("Date:"))
            .map(|line| line.trim_start_matches("Date:").trim().to_string())
    });

    let to_emails = split_display_addresses(&display_to);
    let cc_emails = split_display_addresses(&display_cc);
    let bcc_emails = split_display_addresses(&display_bcc);

    let plain_text = body.filter(|s| !s.is_empty());
    let html_content = html_body.filter(|s| !s.is_empty());

    let cleaned_text = if let Some(ref plain) = plain_text {
        plain.clone()
    } else if let Some(ref html) = html_content {
        clean_html_content(html)
    } else {
        String::new()
    };

    // --- attachment storages -----------------------------------------------

    let attach_paths: Vec<String> = comp
        .walk()
        .filter(|e| e.is_storage() && e.name().starts_with("__attach_"))
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();

    let mut attachments = Vec::with_capacity(attach_paths.len());
    for path in &attach_paths {
        let long_name = read_msg_string_prop(comp, path, 0x3707); // PR_ATTACH_LONG_FILENAME
        let short_name = read_msg_string_prop(comp, path, 0x3704); // PR_ATTACH_FILENAME
        let display_name = read_msg_string_prop(comp, path, 0x3001); // PR_DISPLAY_NAME
        let extension = read_msg_string_prop(comp, path, 0x3703); // PR_ATTACH_EXTENSION
        let mime_tag = read_msg_string_prop(comp, path, 0x370E); // PR_ATTACH_MIME_TAG

        let filename = long_name
            .or(short_name)
            .or_else(|| display_name.clone())
            .or_else(|| extension.map(|ext| format!("attachment{ext}")));

        // Read binary attachment data directly — no hex encoding.
        let bin_path = format!("{path}/__substg1.0_37010102");
        let binary_data = read_msg_stream(comp, &bin_path);
        let size = binary_data.as_ref().map(Vec::len);
        let att_data = binary_data.map(Bytes::from);

        let mime_type = mime_tag
            .filter(|s| !s.is_empty())
            .or_else(|| Some("application/octet-stream".to_string()));
        let is_image = mime_type.as_ref().map(|m| is_image_mime_type(m)).unwrap_or(false);

        attachments.push(EmailAttachment {
            name: filename.clone(),
            filename,
            mime_type,
            size,
            is_image,
            data: att_data,
        });
    }

    // --- metadata ----------------------------------------------------------

    let mut metadata = HashMap::new();
    if let Some(ref subj) = subject {
        metadata.insert("subject".to_string(), subj.to_string());
    }
    if let Some(ref from) = from_email {
        metadata.insert("email_from".to_string(), from.to_string());
    }
    if let Some(ref name) = sender_name {
        if !name.is_empty() {
            metadata.insert("from_name".to_string(), name.to_string());
        }
    }
    if !to_emails.is_empty() {
        metadata.insert("email_to".to_string(), to_emails.join(", "));
    }
    if !cc_emails.is_empty() {
        metadata.insert("email_cc".to_string(), cc_emails.join(", "));
    }
    if !bcc_emails.is_empty() {
        metadata.insert("email_bcc".to_string(), bcc_emails.join(", "));
    }
    if let Some(ref dt) = date {
        metadata.insert("date".to_string(), dt.to_string());
    }
    if let Some(ref msg_id) = message_id {
        metadata.insert("message_id".to_string(), msg_id.to_string());
    }

    Ok(EmailExtractionResult {
        subject,
        from_email,
        to_emails,
        cc_emails,
        bcc_emails,
        date,
        message_id,
        plain_text,
        html_content,
        cleaned_text,
        attachments,
        metadata,
    })
}

// --- MSG / CFB helper functions --------------------------------------------

/// Read a raw CFB stream by path; returns `None` for missing or empty streams.
fn read_msg_stream<F: std::io::Read + std::io::Seek>(comp: &mut cfb::CompoundFile<F>, path: &str) -> Option<Vec<u8>> {
    use std::io::Read;
    let mut stream = comp.open_stream(path).ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    if buf.is_empty() { None } else { Some(buf) }
}

/// Read a MAPI string property (tries PT_UNICODE then PT_STRING8).
fn read_msg_string_prop<F: std::io::Read + std::io::Seek>(
    comp: &mut cfb::CompoundFile<F>,
    base: &str,
    prop_id: u16,
) -> Option<String> {
    // Try PT_UNICODE (001F) first.
    let unicode_path = format!("{base}/__substg1.0_{prop_id:04X}001F");
    if let Some(buf) = read_msg_stream(comp, &unicode_path) {
        return Some(decode_utf16le_bytes(&buf));
    }
    // Fallback to PT_STRING8 (001E).
    let ansi_path = format!("{base}/__substg1.0_{prop_id:04X}001E");
    read_msg_stream(comp, &ansi_path).map(|buf| String::from_utf8_lossy(&buf).into_owned())
}

/// Decode UTF-16LE bytes to a String, stripping trailing NUL chars.
fn decode_utf16le_bytes(data: &[u8]) -> String {
    let u16s: Vec<u16> = data.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    String::from_utf16_lossy(&u16s).trim_end_matches('\0').to_string()
}

/// Split semicolon/comma-separated display addresses into individual strings.
fn split_display_addresses(display: &Option<String>) -> Vec<String> {
    display
        .as_deref()
        .unwrap_or("")
        .split([';', ','])
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Extract email content from either .eml or .msg format
pub fn extract_email_content(data: &[u8], mime_type: &str) -> Result<EmailExtractionResult> {
    if data.is_empty() {
        return Err(KreuzbergError::validation("Email content is empty".to_string()));
    }

    match mime_type {
        "message/rfc822" | "text/plain" => parse_eml_content(data),
        "application/vnd.ms-outlook" => parse_msg_content(data),
        _ => Err(KreuzbergError::validation(format!(
            "Unsupported email MIME type: {}",
            mime_type
        ))),
    }
}

/// Build text output from email extraction result
pub fn build_email_text_output(result: &EmailExtractionResult) -> String {
    let mut text_parts = Vec::with_capacity(10);

    if let Some(ref subject) = result.subject {
        text_parts.push(format!("Subject: {}", subject));
    }

    if let Some(ref from) = result.from_email {
        text_parts.push(format!("From: {}", from));
    }

    if !result.to_emails.is_empty() {
        text_parts.push(format!("To: {}", result.to_emails.join(", ")));
    }

    if !result.cc_emails.is_empty() {
        text_parts.push(format!("CC: {}", result.cc_emails.join(", ")));
    }

    if !result.bcc_emails.is_empty() {
        text_parts.push(format!("BCC: {}", result.bcc_emails.join(", ")));
    }

    if let Some(ref date) = result.date {
        text_parts.push(format!("Date: {}", date));
    }

    text_parts.push(result.cleaned_text.clone());

    if !result.attachments.is_empty() {
        let mut attachment_names = Vec::with_capacity(result.attachments.len().min(20));
        for att in &result.attachments {
            if let Some(name) = att.name.as_ref().or(att.filename.as_ref()) {
                attachment_names.push(name.clone());
            }
        }
        if !attachment_names.is_empty() {
            text_parts.push(format!("Attachments: {}", attachment_names.join(", ")));
        }
    }

    text_parts.join("\n")
}

fn clean_html_content(html: &str) -> String {
    if html.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(html.len().saturating_mul(8).saturating_div(10));

    let cleaned = script_regex().replace_all(html, "");
    let cleaned = style_regex().replace_all(&cleaned, "");
    let cleaned = html_tag_regex().replace_all(&cleaned, "");
    let cleaned = whitespace_regex().replace_all(&cleaned, " ");

    result.push_str(cleaned.trim());
    result
}

fn is_image_mime_type(mime_type: &str) -> bool {
    mime_type.starts_with("image/")
}

fn parse_content_type(content_type: &str) -> String {
    let trimmed = content_type.trim();
    if trimmed.is_empty() {
        return "application/octet-stream".to_string();
    }
    trimmed
        .split(';')
        .next()
        .unwrap_or("application/octet-stream")
        .trim()
        .to_lowercase()
}

#[allow(clippy::too_many_arguments)]
fn build_metadata(
    subject: &Option<String>,
    from_email: &Option<String>,
    to_emails: &[String],
    cc_emails: &[String],
    bcc_emails: &[String],
    date: &Option<String>,
    message_id: &Option<String>,
    attachments: &[EmailAttachment],
) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    if let Some(subj) = subject {
        metadata.insert("subject".to_string(), subj.clone());
    }
    if let Some(from) = from_email {
        metadata.insert("email_from".to_string(), from.clone());
    }
    if !to_emails.is_empty() {
        metadata.insert("email_to".to_string(), to_emails.join(", "));
    }
    if !cc_emails.is_empty() {
        metadata.insert("email_cc".to_string(), cc_emails.join(", "));
    }
    if !bcc_emails.is_empty() {
        metadata.insert("email_bcc".to_string(), bcc_emails.join(", "));
    }
    if let Some(dt) = date {
        metadata.insert("date".to_string(), dt.clone());
    }
    if let Some(msg_id) = message_id {
        metadata.insert("message_id".to_string(), msg_id.clone());
    }

    if !attachments.is_empty() {
        let attachment_names: Vec<String> = attachments
            .iter()
            .filter_map(|att| att.name.as_ref().or(att.filename.as_ref()))
            .cloned()
            .collect();
        if !attachment_names.is_empty() {
            metadata.insert("attachments".to_string(), attachment_names.join(", "));
        }
    }

    metadata
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_html_content() {
        let html = "<p>Hello <b>World</b></p>";
        let cleaned = clean_html_content(html);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_clean_html_with_whitespace() {
        let html = "<div>  Multiple   \n  spaces  </div>";
        let cleaned = clean_html_content(html);
        assert_eq!(cleaned, "Multiple spaces");
    }

    #[test]
    fn test_clean_html_with_script_and_style() {
        let html = r#"
            <html>
                <head><style>body { color: red; }</style></head>
                <body>
                    <script>alert('test');</script>
                    <p>Hello World</p>
                </body>
            </html>
        "#;
        let cleaned = clean_html_content(html);
        assert!(!cleaned.contains("<script>"));
        assert!(!cleaned.contains("<style>"));
        assert!(cleaned.contains("Hello World"));
    }

    #[test]
    fn test_is_image_mime_type() {
        assert!(is_image_mime_type("image/png"));
        assert!(is_image_mime_type("image/jpeg"));
        assert!(!is_image_mime_type("text/plain"));
        assert!(!is_image_mime_type("application/pdf"));
    }

    #[test]
    fn test_parse_content_type() {
        assert_eq!(parse_content_type("text/plain"), "text/plain");
        assert_eq!(parse_content_type("text/plain; charset=utf-8"), "text/plain");
        assert_eq!(parse_content_type("image/jpeg; name=test.jpg"), "image/jpeg");
        assert_eq!(parse_content_type(""), "application/octet-stream");
    }

    #[test]
    fn test_extract_email_content_empty_data() {
        let result = extract_email_content(b"", "message/rfc822");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KreuzbergError::Validation { .. }));
    }

    #[test]
    fn test_extract_email_content_invalid_mime_type() {
        let result = extract_email_content(b"test", "application/pdf");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KreuzbergError::Validation { .. }));
    }

    #[test]
    fn test_parse_eml_content_invalid() {
        let result = parse_eml_content(b"not an email");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_msg_content_invalid() {
        let result = parse_msg_content(b"not a msg file");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), KreuzbergError::Parsing { .. }));
    }

    #[test]
    fn test_simple_eml_parsing() {
        let eml_content =
            b"From: test@example.com\r\nTo: recipient@example.com\r\nSubject: Test Email\r\n\r\nThis is a test email body.";

        let result = parse_eml_content(eml_content).unwrap();
        assert_eq!(result.subject, Some("Test Email".to_string()));
        assert_eq!(result.from_email, Some("test@example.com".to_string()));
        assert_eq!(result.to_emails, vec!["recipient@example.com".to_string()]);
        assert_eq!(result.cleaned_text, "This is a test email body.");
    }

    #[test]
    fn test_build_email_text_output_minimal() {
        let result = EmailExtractionResult {
            subject: Some("Test".to_string()),
            from_email: Some("sender@example.com".to_string()),
            to_emails: vec!["recipient@example.com".to_string()],
            cc_emails: vec![],
            bcc_emails: vec![],
            date: None,
            message_id: None,
            plain_text: None,
            html_content: None,
            cleaned_text: "Hello World".to_string(),
            attachments: vec![],
            metadata: HashMap::new(),
        };

        let output = build_email_text_output(&result);
        assert!(output.contains("Subject: Test"));
        assert!(output.contains("From: sender@example.com"));
        assert!(output.contains("To: recipient@example.com"));
        assert!(output.contains("Hello World"));
    }

    #[test]
    fn test_build_email_text_output_with_attachments() {
        let result = EmailExtractionResult {
            subject: Some("Test".to_string()),
            from_email: Some("sender@example.com".to_string()),
            to_emails: vec!["recipient@example.com".to_string()],
            cc_emails: vec![],
            bcc_emails: vec![],
            date: None,
            message_id: None,
            plain_text: None,
            html_content: None,
            cleaned_text: "Hello World".to_string(),
            attachments: vec![EmailAttachment {
                name: Some("file.txt".to_string()),
                filename: Some("file.txt".to_string()),
                mime_type: Some("text/plain".to_string()),
                size: Some(1024),
                is_image: false,
                data: None,
            }],
            metadata: HashMap::new(),
        };

        let output = build_email_text_output(&result);
        assert!(output.contains("Attachments: file.txt"));
    }

    #[test]
    fn test_build_metadata() {
        let subject = Some("Test Subject".to_string());
        let from_email = Some("sender@example.com".to_string());
        let to_emails = vec!["recipient@example.com".to_string()];
        let cc_emails = vec!["cc@example.com".to_string()];
        let bcc_emails = vec!["bcc@example.com".to_string()];
        let date = Some("2024-01-01T12:00:00Z".to_string());
        let message_id = Some("<abc123@example.com>".to_string());
        let attachments = vec![];

        let metadata = build_metadata(
            &subject,
            &from_email,
            &to_emails,
            &cc_emails,
            &bcc_emails,
            &date,
            &message_id,
            &attachments,
        );

        assert_eq!(metadata.get("subject"), Some(&"Test Subject".to_string()));
        assert_eq!(metadata.get("email_from"), Some(&"sender@example.com".to_string()));
        assert_eq!(metadata.get("email_to"), Some(&"recipient@example.com".to_string()));
        assert_eq!(metadata.get("email_cc"), Some(&"cc@example.com".to_string()));
        assert_eq!(metadata.get("email_bcc"), Some(&"bcc@example.com".to_string()));
        assert_eq!(metadata.get("date"), Some(&"2024-01-01T12:00:00Z".to_string()));
        assert_eq!(metadata.get("message_id"), Some(&"<abc123@example.com>".to_string()));
    }

    #[test]
    fn test_build_metadata_with_attachments() {
        let attachments = vec![
            EmailAttachment {
                name: Some("file1.pdf".to_string()),
                filename: Some("file1.pdf".to_string()),
                mime_type: Some("application/pdf".to_string()),
                size: Some(1024),
                is_image: false,
                data: None,
            },
            EmailAttachment {
                name: Some("image.png".to_string()),
                filename: Some("image.png".to_string()),
                mime_type: Some("image/png".to_string()),
                size: Some(2048),
                is_image: true,
                data: None,
            },
        ];

        let metadata = build_metadata(&None, &None, &[], &[], &[], &None, &None, &attachments);

        assert_eq!(metadata.get("attachments"), Some(&"file1.pdf, image.png".to_string()));
    }

    #[test]
    fn test_clean_html_content_empty() {
        let cleaned = clean_html_content("");
        assert_eq!(cleaned, "");
    }

    #[test]
    fn test_clean_html_content_only_tags() {
        let html = "<div><span><p></p></span></div>";
        let cleaned = clean_html_content(html);
        assert_eq!(cleaned, "");
    }

    #[test]
    fn test_clean_html_content_nested_tags() {
        let html = "<div><p>Outer <span>Inner <b>Bold</b></span> Text</p></div>";
        let cleaned = clean_html_content(html);
        assert_eq!(cleaned, "Outer Inner Bold Text");
    }

    #[test]
    fn test_clean_html_content_multiple_scripts() {
        let html = r#"
            <script>function a() {}</script>
            <p>Content</p>
            <script>function b() {}</script>
        "#;
        let cleaned = clean_html_content(html);
        assert!(!cleaned.contains("function"));
        assert!(cleaned.contains("Content"));
    }

    #[test]
    fn test_is_image_mime_type_variants() {
        assert!(is_image_mime_type("image/gif"));
        assert!(is_image_mime_type("image/webp"));
        assert!(is_image_mime_type("image/svg+xml"));
        assert!(!is_image_mime_type("video/mp4"));
        assert!(!is_image_mime_type("audio/mp3"));
    }

    #[test]
    fn test_parse_content_type_with_parameters() {
        assert_eq!(parse_content_type("multipart/mixed; boundary=xyz"), "multipart/mixed");
        assert_eq!(parse_content_type("text/html; charset=UTF-8"), "text/html");
    }

    #[test]
    fn test_parse_content_type_whitespace() {
        assert_eq!(parse_content_type("  text/plain  "), "text/plain");
        assert_eq!(parse_content_type(" text/plain ; charset=utf-8 "), "text/plain");
    }

    #[test]
    fn test_parse_content_type_case_insensitive() {
        assert_eq!(parse_content_type("TEXT/PLAIN"), "text/plain");
        assert_eq!(parse_content_type("Image/JPEG"), "image/jpeg");
    }

    #[test]
    fn test_extract_email_content_mime_variants() {
        let eml_content = b"From: test@example.com\r\n\r\nBody";

        assert!(extract_email_content(eml_content, "message/rfc822").is_ok());
        assert!(extract_email_content(eml_content, "text/plain").is_ok());
    }

    #[test]
    fn test_simple_eml_with_multiple_recipients() {
        let eml_content = b"From: sender@example.com\r\nTo: r1@example.com, r2@example.com\r\nCc: cc@example.com\r\nBcc: bcc@example.com\r\nSubject: Multi-recipient\r\n\r\nBody";

        let result = parse_eml_content(eml_content).unwrap();
        assert_eq!(result.to_emails.len(), 2);
        assert!(result.to_emails.contains(&"r1@example.com".to_string()));
        assert!(result.to_emails.contains(&"r2@example.com".to_string()));
    }

    #[test]
    fn test_simple_eml_with_html_body() {
        let eml_content = b"From: sender@example.com\r\nTo: recipient@example.com\r\nSubject: HTML Email\r\nContent-Type: text/html\r\n\r\n<html><body><p>HTML Body</p></body></html>";

        let result = parse_eml_content(eml_content).unwrap();
        assert!(!result.cleaned_text.is_empty());
    }

    #[test]
    fn test_build_email_text_output_with_all_fields() {
        let result = EmailExtractionResult {
            subject: Some("Complete Email".to_string()),
            from_email: Some("sender@example.com".to_string()),
            to_emails: vec!["recipient@example.com".to_string()],
            cc_emails: vec!["cc@example.com".to_string()],
            bcc_emails: vec!["bcc@example.com".to_string()],
            date: Some("2024-01-01T12:00:00Z".to_string()),
            message_id: Some("<msg123@example.com>".to_string()),
            plain_text: Some("Plain text body".to_string()),
            html_content: Some("<html><body>HTML body</body></html>".to_string()),
            cleaned_text: "Cleaned body text".to_string(),
            attachments: vec![],
            metadata: HashMap::new(),
        };

        let output = build_email_text_output(&result);
        assert!(output.contains("Subject: Complete Email"));
        assert!(output.contains("From: sender@example.com"));
        assert!(output.contains("To: recipient@example.com"));
        assert!(output.contains("CC: cc@example.com"));
        assert!(output.contains("BCC: bcc@example.com"));
        assert!(output.contains("Date: 2024-01-01T12:00:00Z"));
        assert!(output.contains("Cleaned body text"));
    }

    #[test]
    fn test_build_email_text_output_empty_attachments() {
        let result = EmailExtractionResult {
            subject: Some("Test".to_string()),
            from_email: Some("sender@example.com".to_string()),
            to_emails: vec!["recipient@example.com".to_string()],
            cc_emails: vec![],
            bcc_emails: vec![],
            date: None,
            message_id: None,
            plain_text: None,
            html_content: None,
            cleaned_text: "Body".to_string(),
            attachments: vec![EmailAttachment {
                name: None,
                filename: None,
                mime_type: Some("application/octet-stream".to_string()),
                size: Some(100),
                is_image: false,
                data: None,
            }],
            metadata: HashMap::new(),
        };

        let output = build_email_text_output(&result);
        assert!(output.contains("Body"));
    }

    #[test]
    fn test_build_metadata_empty_fields() {
        let metadata = build_metadata(&None, &None, &[], &[], &[], &None, &None, &[]);
        assert!(metadata.is_empty());
    }

    #[test]
    fn test_build_metadata_partial_fields() {
        let subject = Some("Test".to_string());
        let date = Some("2024-01-01".to_string());

        let metadata = build_metadata(&subject, &None, &[], &[], &[], &date, &None, &[]);

        assert_eq!(metadata.get("subject"), Some(&"Test".to_string()));
        assert_eq!(metadata.get("date"), Some(&"2024-01-01".to_string()));
        assert_eq!(metadata.len(), 2);
    }

    #[test]
    fn test_clean_html_content_case_insensitive_tags() {
        let html = "<SCRIPT>code</SCRIPT><STYLE>css</STYLE><P>Text</P>";
        let cleaned = clean_html_content(html);
        assert!(!cleaned.contains("code"));
        assert!(!cleaned.contains("css"));
        assert!(cleaned.contains("Text"));
    }

    #[test]
    fn test_simple_eml_with_date() {
        let eml_content = b"From: sender@example.com\r\nTo: recipient@example.com\r\nDate: Mon, 1 Jan 2024 12:00:00 +0000\r\nSubject: Test\r\n\r\nBody";

        let result = parse_eml_content(eml_content).unwrap();
        assert!(result.date.is_some());
    }

    #[test]
    fn test_simple_eml_with_message_id() {
        let eml_content = b"From: sender@example.com\r\nTo: recipient@example.com\r\nMessage-ID: <unique@example.com>\r\nSubject: Test\r\n\r\nBody";

        let result = parse_eml_content(eml_content).unwrap();
        assert!(result.message_id.is_some());
    }

    #[test]
    fn test_simple_eml_minimal() {
        let eml_content = b"From: sender@example.com\r\n\r\nMinimal body";

        let result = parse_eml_content(eml_content).unwrap();
        assert_eq!(result.from_email, Some("sender@example.com".to_string()));
        assert_eq!(result.cleaned_text, "Minimal body");
    }

    #[test]
    fn test_regex_initialization() {
        let _ = html_tag_regex();
        let _ = script_regex();
        let _ = style_regex();
        let _ = whitespace_regex();

        let _ = html_tag_regex();
        let _ = script_regex();
        let _ = style_regex();
        let _ = whitespace_regex();
    }
}
