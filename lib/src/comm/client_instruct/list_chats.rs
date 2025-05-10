use rst_proc_macro::TryIntoClientMessage;
use serde::{Deserialize, Serialize};

#[derive(TryIntoClientMessage)]
#[derive(Serialize, Deserialize)]
pub struct ListChats { }

impl ListChats {
    pub fn new(
    ) -> Self {
        ListChats { }
    }
}
