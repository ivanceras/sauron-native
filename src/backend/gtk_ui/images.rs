use gdk_pixbuf::{PixbufLoader, PixbufLoaderExt};
use gtk::{Image, ImageExt};
use image::ImageFormat;

pub fn svg_image(bytes: &[u8]) -> Image {
    let image = Image::new();
    let pixbuf_loader = PixbufLoader::new_with_mime_type("image/svg+xml")
        .expect("error loader");
    pixbuf_loader
        .write(bytes)
        .expect("Unable to write svg data into pixbuf_loader");

    pixbuf_loader.close().expect("error creating pixbuf");

    let pixbuf = pixbuf_loader.get_pixbuf();

    image.set_from_pixbuf(Some(&pixbuf.expect("error in pixbuf_loader")));
    image
}

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
