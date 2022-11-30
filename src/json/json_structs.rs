use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct AccountList {
    pub(crate) account_list: Vec<Account>

}

#[derive(Deserialize, Serialize, Debug, Default,PartialEq)]
pub struct Account {
    pub(crate) website: String,
    pub(crate) username: String,
    pub(crate) password: Vec<u8>,
    pub(crate) tag: Vec<u8>
}


