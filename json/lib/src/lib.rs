use serde_derive::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub account_name: String,
    pub balance: u32,
}
