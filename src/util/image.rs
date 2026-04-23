

pub enum ImageFormat {
    RGB,
    RGBA,
}

pub enum ImageType {
    PNG,
    JPG,
    BMP,
    TGA,
    TIFF,
}

struct Image {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

pub fn load_image(path: &str) -> Option<Image> {
    match image::open(path) {
        Ok(img) => {
            let rgba = img.to_rgba8();
            Some(Image {
                width: rgba.width(),
                height: rgba.height(),
                data: rgba.into_raw(),
            })
        }
        Err(_) => None,
    }
}
