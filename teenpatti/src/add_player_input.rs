use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use crate::card::Card;

//Helper structs
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddPlayerInput {
    pub account_id: String,
    pub name: String,
    pub cards: Vec<Card>,
}