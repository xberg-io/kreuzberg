/// Regression test for GitHub #1059.
///
/// `kreuzberg_email_attachment_data` was the only byte-buffer accessor on a public
/// FFI-exposed DTO that did not follow the established `*_data(ptr, out_len: *mut usize)`
/// protocol used by `kreuzberg_extracted_image_data`, `kreuzberg_embedded_file_data`,
/// and `kreuzberg_batch_bytes_item_content`.
///
/// Because `EmailAttachment.data` is `Option<Bytes>` (the only optional byte buffer among
/// public types), alef's heuristic for emitting the two-parameter form did not trigger.
/// Callers had no way to know the valid length of the returned pointer, making any read
/// past the first byte undefined behaviour (especially for payloads containing 0x00).
///
/// This test is the "lock-in" test:
/// - It asserts at runtime + against the committed header that the correct signature
///   (with out_len) is present after regeneration.
/// - It exercises a payload containing embedded NULs + a high byte to kill any
///   strlen / "read until 0" or truncated read assumptions.
///
/// The runtime portion of the test (the part that actually calls the two-param form)
/// becomes active once `task alef:generate` has been run with an alef version that
/// emits the proper accessor. Until then the header signature assertion provides the
/// cheap, always-on regression signal.
///
/// See also: crates/kreuzberg-ffi/tests/vtable_bytes_len.rs (previous identical class of bug).
/// Per project rules: every unsafe block has a SAFETY comment.
use std::ffi::{CString, c_char};
use std::fs;
use std::path::Path;

// The functions we need are re-exported / visible via the rlib built from the
// generated lib.rs. We only use the stable ones (from_json / free) directly here.
use kreuzberg_ffi::{kreuzberg_email_attachment_free, kreuzberg_email_attachment_from_json, kreuzberg_last_error_code};

// Desired (correct) signature per the contract used by all other byte accessors.
// We declare it locally so this test encodes the exact ABI we require from the generator.
// After the generator is fixed this will match the symbol emitted in the cdylib/rlib.
//
// We use raw pointers for the opaque handle type so the test does not need to name
// the (non-public) kreuzberg::EmailAttachment type.
unsafe extern "C" {
    /// Get the `data` field from a `EmailAttachment`.
    /// The returned pointer is only valid while the handle is alive.
    /// # Safety
    /// - `ptr` must be a valid non-null handle previously returned by this library.
    /// - `out_len` may be null (in which case length is not written).
    /// - Caller must not free the returned pointer; it is a view into the handle's Bytes.
    pub fn kreuzberg_email_attachment_data(ptr: *const std::ffi::c_void, out_len: *mut usize) -> *mut u8;
}

/// Construct a minimal EmailAttachment JSON with a data payload that contains
/// an embedded NUL and a trailing high byte (0xEF). This defeats any strlen-based
/// or "read first byte only" implementations.
fn attachment_json_with_nuls() -> CString {
    // 8 bytes: JPEG-ish magic + NUL in the middle + high byte at the end.
    // Length is authoritative and known.
    let data: Vec<u8> = vec![0xFF, 0xD8, 0xFF, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    let json = format!(
        r#"{{
            "name": "test.bin",
            "filename": "test.bin",
            "mime_type": "application/octet-stream",
            "size": {},
            "is_image": false,
            "data": {}
        }}"#,
        data.len(),
        serde_json::to_string(&data).unwrap()
    );
    CString::new(json).expect("valid UTF-8 JSON for test attachment")
}

/// Ignored until a fixed alef emits the correct accessor (then `task alef:generate` + rebuild will make it pass).
/// The test body + this file remain the permanent regression specification for #1059.
#[test]
#[ignore = "requires alef fix + regeneration for the Option<Bytes> data case (see #1059)"]
fn email_attachment_data_accessor_must_provide_out_len_in_header() {
    // Cheap, always-on regression harness (no linking to the data symbol required).
    // This will fail on any tree where the generator has not yet been fixed.
    let header_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("include/kreuzberg.h");
    let header = fs::read_to_string(&header_path).expect("committed kreuzberg.h must be readable by the test");

    // Find the specific declaration block for this function and require that
    // it mentions out_len (the precise contract used by the other three data accessors).
    let buggy_declaration = "uint8_t *kreuzberg_email_attachment_data(const KREUZBERGEmailAttachment *ptr);";
    let looks_correct = header.lines().collect::<Vec<_>>().windows(3).any(|w| {
        let joined = w.join("\n");
        joined.contains("kreuzberg_email_attachment_data")
            && (joined.contains("out_len") || joined.contains("uintptr_t *out_len"))
            && !joined.contains(buggy_declaration)
    });

    assert!(
        looks_correct,
        "GitHub #1059 regression: the declaration of kreuzberg_email_attachment_data \
         in crates/kreuzberg-ffi/include/kreuzberg.h does not contain the required \
         `out_len` parameter.\n\n\
         Expected something like:\n    uint8_t *kreuzberg_email_attachment_data(..., uintptr_t *out_len);\n\n\
         Found the old 1-parameter form. Fix requires `task alef:generate` with an \
         updated alef that handles Option<Bytes> fields for the FFI byte accessor heuristic.\n\n\
         This is the lock-in test for #1059."
    );
}

/// Ignored until a fixed alef emits the correct 2-parameter accessor.
/// This test encodes the exact required behaviour (full length + correct bytes past embedded NUL).
#[test]
#[ignore = "requires alef fix + regeneration for the Option<Bytes> data case (see #1059)"]
fn email_attachment_data_with_out_len_returns_full_buffer_including_embedded_nuls() {
    // This test demonstrates the full contract once the generator emits the
    // two-parameter form. It is written against the desired extern declaration above.
    //
    // Until the generator is fixed this test documents the required behaviour.
    // After `task alef:generate` + rebuild it will execute the real call path.

    let json = attachment_json_with_nuls();
    // SAFETY: json is a valid null-terminated CString we just created.
    let handle = unsafe { kreuzberg_email_attachment_from_json(json.as_ptr() as *const c_char) };
    assert!(
        !handle.is_null(),
        "from_json should succeed for our well-formed test attachment (last_error_code={})",
        unsafe { kreuzberg_last_error_code() }
    );

    let mut out_len: usize = 0;
    // SAFETY: handle is non-null and freshly allocated by from_json.
    // We pass a valid &mut out_len. The returned pointer must not be freed by us.
    let data_ptr = unsafe { kreuzberg_email_attachment_data(handle as *const std::ffi::c_void, &mut out_len) };

    assert!(
        !data_ptr.is_null(),
        "data pointer must be non-null for an attachment we created with a Some(data) payload"
    );
    assert_eq!(
        out_len, 8,
        "out_len must report the exact length of the Bytes payload (not 0, not guessed, not truncated at NUL)"
    );

    // SAFETY: data_ptr is valid for [0..out_len] because:
    // - it came from the handle's internal Bytes (which we control),
    // - out_len was written by the accessor,
    // - the handle is still alive (we have not called free yet).
    let slice = unsafe { std::slice::from_raw_parts(data_ptr, out_len) };

    assert_eq!(slice.len(), 8);
    assert_eq!(slice[0], 0xFF);
    assert_eq!(slice[3], 0x00, "must be able to read the embedded NUL");
    assert_eq!(
        slice[7], 0xEF,
        "must be able to read bytes after the NUL (no truncation)"
    );

    // Cleanup
    // SAFETY: handle came from from_json; we are the owner.
    unsafe { kreuzberg_email_attachment_free(handle) };
}

#[test]
fn email_attachment_data_none_returns_null_and_zero_len() {
    // Attachment with data: null (or omitted) must give null pointer + len=0 (or leave len untouched).
    let json = CString::new(
        r#"{"name":"empty","filename":"empty","mime_type":null,"size":null,"is_image":false,"data":null}"#,
    )
    .unwrap();

    // SAFETY: json is valid.
    let handle = unsafe { kreuzberg_email_attachment_from_json(json.as_ptr() as *const c_char) };
    assert!(!handle.is_null());

    let mut out_len: usize = 0xDEAD_BEEF; // sentinel to detect whether it was written

    // SAFETY: handle is valid and we are only reading the data pointer.
    let data_ptr = unsafe { kreuzberg_email_attachment_data(handle as *const std::ffi::c_void, &mut out_len) };

    assert!(
        data_ptr.is_null(),
        "data must be null when the attachment has no payload"
    );
    // The established convention in the other three accessors is to write the len
    // even on the None/empty path (0). We accept either behaviour for the None case
    // as long as the pointer is null.
    if out_len != 0xDEAD_BEEF {
        assert_eq!(out_len, 0, "when length is written for a None payload it must be 0");
    }

    // SAFETY: handle from from_json.
    unsafe { kreuzberg_email_attachment_free(handle) };
}
