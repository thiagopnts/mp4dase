use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Track {
    pub track_id: u32,
    pub duration: u64,
    pub format: String,
    pub kind: Content,
}

#[derive(Debug, Clone)]
pub enum Content {
    Video,
    Audio,
    Text,
    Other(String),
}

impl From<&[u8; 4]> for Content {
    fn from(s: &[u8; 4]) -> Self {
        match s {
            b"vide" => Content::Video,
            b"soun" => Content::Audio,
            b"text" => Content::Text,
            _ => Content::Other(String::from_utf8_lossy(s).to_string()),
        }
    }
}

impl FromStr for Content {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vide" => Ok(Content::Video),
            "soun" => Ok(Content::Audio),
            "text" => Ok(Content::Text),
            _ => Ok(Content::Other(s.to_string())),
        }
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Content::Video => write!(f, "Video"),
            Content::Audio => write!(f, "Audio"),
            Content::Text => write!(f, "Text"),
            Content::Other(s) => write!(f, "{}", s),
        }
    }
}
