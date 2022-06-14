use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    pub card_type: String,
    pub suit: String,
    pub value: u16,
}