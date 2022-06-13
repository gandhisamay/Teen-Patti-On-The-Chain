use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    pub card_type: String,
    pub suit: String,
    pub value: u16,
}