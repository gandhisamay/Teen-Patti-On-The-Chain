use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum SuitType {
    Spade,
    Heart,
    Club,
    Diamond,
}
