use image::ImageFormat;

/// get the mime type of image
pub(crate) fn image_mime_type(bytes: &[u8]) -> Option<&'static str> {
    let img_format =
        image::guess_format(bytes).expect("must have an image format");
    match img_format {
        ImageFormat::Png => Some("image/png"),
        ImageFormat::Jpeg => Some("image/jpeg"),
        _ => None,
    }
}
