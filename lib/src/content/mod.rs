use std::fmt::Display;
use serde::{Serialize, Deserialize};

mod image;
pub use image::*;

#[derive(Clone, Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Image,
    Audio,
    Video,
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Text => write!(f, "Text"),
            ContentType::Image => write!(f, "Image"),
            ContentType::Audio => write!(f, "Audio"),
            ContentType::Video => write!(f, "Video"),
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Content {
    /// The content type
    pub ctype: ContentType,
    pub data: Vec<u8>,
}

impl Content {
    pub fn new(r#type: ContentType, data: Vec<u8>) -> Self {
        Content {
            ctype: r#type,
            data,
        }
    }

    pub fn new_text(data: String) -> Self {
        Content {
            ctype: ContentType::Text,
            data: data.into_bytes(),
        }
    }

    // TODO: add more content types
}
