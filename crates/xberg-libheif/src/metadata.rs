use four_cc::FourCC;

pub struct ImageMetadata {
    /// An instance of `FourCC` indicating the type of the metadata,
    /// as specified in the HEIF file.
    ///
    /// Exif data will have the type `b"Exif"`.
    pub item_type: FourCC,
    /// For EXIF, the content type is `""`.
    ///
    /// For XMP, the content type is `"application/rdf+xml"`.
    pub content_type: String,
    /// An absolute URI. Only valid for item_type == "uri".
    pub uri_type: String,
    /// The data is exactly as stored in the HEIF file.
    ///
    /// For Exif data, you probably have to skip the first four bytes of
    /// the data, since they indicate the offset to the start of
    /// the TIFF header of the Exif data.
    pub raw_data: Vec<u8>,
}
