use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};

// 5 Ⓝ in yoctoNEAR
const INITIAL_BET: u128 = 5_000_000_000_000_000_000_000_000;

const SUITS: [&str; 4] = ["Spade", "Heart", "Club", "Diamond"];

const CARD_TYPES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

// const VALUES:LookupMap<String,u16> = LookupMap::new(key_prefix: S)
// need an efficient datastructure

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(&self) {
        // generate the 52 cards here
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
enum PlayerActions {
    Fold,
    Raise { raise_amount: f64 },
    Show { players_remaining: u16 }, //only when two players are remaining
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
enum SuitType {
    Spade,
    Heart,
    Club,
    Diamond,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    card_type: String,
    suit: SuitType,
    value: u16,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
    account_id: AccountId,
    cards: Vec<Card>,
    name: String,
    betting_amount: f64,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Game {
    players: Vec<Player>,
    folded: Vec<Player>,
    tokens_staked: f64,
}

#[near_bindgen]
impl Game {
    pub fn start_game() {
        println!("Welcome to Teen Patti");
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
