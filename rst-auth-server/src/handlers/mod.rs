mod query_uid;
pub use query_uid::*;
mod auth;
pub use auth::*;
mod metadata;
pub use metadata::*;
mod register;
pub use register::*;

const EXPIRATION_TIME: u64 = 60 * 60;
