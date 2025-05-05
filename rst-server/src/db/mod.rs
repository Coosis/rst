pub const MONGODB_URI: &str = "mongodb://localhost:27017";
pub const CACHE_URL: &str = "redis://localhost:6379";
pub const DB: &str = "test";
pub const TB_USERS: &str = "users";
pub const TB_MESSAGES: &str = "messages";
pub const TB_CHATS: &str = "chats";
pub const TB_INVITES: &str = "invites";
pub const SECRET: &[u8] = "secret".as_bytes();

mod find;
pub use find::*;
mod insert;
pub use insert::*;
mod update;
pub use update::*;
