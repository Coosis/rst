use axum::response::IntoResponse;
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use bson::{Binary, Bson};
use bson::spec::BinarySubtype;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Deref;
use std::str::FromStr;

/// Exists to bridge the gap between `uuid::Uuid` and `bson::Binary`.
/// Basically a wrapper around `uuid::Uuid` so that mongodb can stop 
/// acting like a child
#[derive(Debug, Clone, Copy)]
pub struct Uuid(pub uuid::Uuid);
impl Uuid {
    pub fn new_v4() -> Self {
        Uuid(uuid::Uuid::new_v4())
    }

    pub fn now_v7() -> Self {
        Uuid(uuid::Uuid::now_v7())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Uuid(uuid::Uuid::from_slice(bytes).unwrap())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    pub fn from_str(s: &str) -> Result<Self, uuid::Error> {
        uuid::Uuid::from_str(s)
            .map(Uuid)
    }

    pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
        uuid::Uuid::parse_str(s)
            .map(Uuid)
    }
}

impl Hash for Uuid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for Uuid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Uuid {}

impl From<Uuid> for Bson {
    fn from(value: Uuid) -> Self {
        Bson::Binary(Binary {
            subtype: BinarySubtype::Uuid,
            bytes: value.0.as_bytes().to_vec(),
        })
    }
}


impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bin = Binary {
            subtype: BinarySubtype::Uuid,
            bytes: self.0.as_bytes().to_vec(),
        };
        Bson::Binary(bin).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bson = Bson::deserialize(deserializer)?;
        if let Bson::Binary(bson::Binary { subtype: BinarySubtype::Uuid, bytes }) = bson {
            Ok(Uuid(uuid::Uuid::from_slice(&bytes).map_err(serde::de::Error::custom)?))
        } else {
            Err(serde::de::Error::custom("expected a BSON binary UUID"))
        }
    }
}
