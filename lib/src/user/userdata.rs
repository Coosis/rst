// use serde::{Deserialize, Serialize};
// use crate::Uuid;
// use crate::comm::client_instruct::RegisterRequest;
// use crate::user::UserCredential;
//
// /// A app user
// #[derive(Serialize, Deserialize)]
// pub struct User {
//     pub id: Uuid,
//     pub email: Option<String>,
//     pub phone: Option<String>,
//     pub username: String,
//     pub passwd_hash: String,
//     /// Last time user was seen, in seconds since epoch
//     pub last_seen: u64,
// }
//
// impl User {
//     pub fn new(
//         id: Uuid,
//         email: Option<String>,
//         phone: Option<String>,
//         username: String,
//         passwd_hash: String,
//         last_seen: u64,
//         ) -> Self {
//         User {
//             id,
//             email,
//             phone,
//             username,
//             passwd_hash,
//             last_seen,
//         }
//     }
//
//     /// Client-side provide inner, 
//     /// server-side create other fields like
//     /// `id` and `last_seen`
//     pub fn with_register_request(
//         inner: RegisterRequest,
//         id: Uuid,
//         hash: String,
//         last_seen: u64,
//         ) -> Self {
//         User {
//             id,
//             email: inner.email,
//             phone: inner.phone,
//             username: inner.username,
//             passwd_hash: hash,
//             last_seen,
//         }
//     }
//
//     pub fn to_public(&self) -> UserCredential {
//         UserCredential::new(
//             self.email.clone(),
//             self.phone.clone(),
//             Some(self.id),
//             self.username.clone(),
//         )
//     }
// }
