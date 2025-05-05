pub enum ImageType {
    PNG,
    JPEG,
}

pub struct Image {
    r#type: ImageType,
    width: u32,
    height: u32,
    data: Vec<u8>,
}
