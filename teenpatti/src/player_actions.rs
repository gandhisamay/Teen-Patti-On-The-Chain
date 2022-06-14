use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerActions {
    Idle,
    Fold,
    Raise(f64),
    Show, //only when two players are remaining
}