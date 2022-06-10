use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::env::{random_seed, random_seed_array};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};
use rand;
use rand::seq::SliceRandom;

// 5 Ⓝ in yoctoNEAR
const INITIAL_BET: u128 = 5_000_000_000_000_000_000_000_000;

const SUITS: [&str; 4] = ["Spade", "Heart", "Club", "Diamond"];

const CARD_TYPES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];
const VALUES: [u16; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

// static KEYWORDS: phf::Map<&'static str, u16> = phf_map! {
//     "loop" => 1,
//     "continue" => 2,
//     "break" => 3,
//     "fn" => 4,
//     "extern" => 5,
// };
// const VALUES:LookupMap<String,u16> = LookupMap::new(key_prefix: S)
// need an efficient datastructure

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        // generate the 52 cards here
        let mut hash_map: LookupMap<String, u16> = LookupMap::new(b"p");

        // generate the lookup map
        for (index, card) in CARD_TYPES.into_iter().enumerate() {
            hash_map.insert(&card.to_string(), &VALUES[index]);
        }

        //generate the deck
        let mut cards: Vec<Card> = Vec::new();
        for suit in SUITS {
            for card_type in CARD_TYPES {
                let card = Card {
                    suit: suit.to_string(),
                    card_type: card_type.to_string(),
                    value: hash_map
                        .get(&card_type.to_string())
                        .expect("index out of bound error"),
                };
                cards.insert(cards.len(), card);
            }
        }

        // shuffle the cards
        // let mut rng = thread_rng();
        // cards.shuffle(&mut rng);

        // generating the deck
        Deck { cards: cards }
    }

    // generates hand for one player
    pub fn generate_hand(&self) -> Vec<Card> {
        // let cards:Vec<Card> = Vec::new();
        let hand: Vec<Card> = self
            .cards
            .choose_multiple(&mut rand::thread_rng(), 3)
            .cloned()
            .collect();
        // Chooses amount elements from the slice at random, without repetition, and in random order.
        hand
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
enum HandType {
    // init field if required
    Trail, //3 of same rank
    PureSequence,
    Sequence,
    Color, //3 cards of the same color,
    Pair,  //2 cards of the same rank
    HighCard,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    card_type: String,
    suit: String,
    value: u16,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
    account_id: AccountId,
    cards: Vec<Card>,
    name: String,
    betting_amount: f64,
    hand_type: HandType,
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
    pub fn start_game() -> Vec<Card> {
        let deck = Deck::new();
        return deck.cards;
    }

    pub fn get_init_amount() -> u128 {
        INITIAL_BET
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
    #[test]
    fn generate_deck() {
        let mut hash_map: LookupMap<String, u16> = LookupMap::new(b"p");

        // generate the lookup map
        for (index, card) in CARD_TYPES.into_iter().enumerate() {
            hash_map.insert(&card.to_string(), &VALUES[index]);
        }

        //generate the deck
        let mut cards: Vec<Card> = Vec::new();
        for suit in SUITS {
            for card_type in CARD_TYPES {
                let card = Card {
                    suit: suit.to_string(),
                    card_type: card_type.to_string(),
                    value: hash_map
                        .get(&card_type.to_string())
                        .expect("index out of bound error"),
                };
                // println!("{:?}", card);
                cards.insert(cards.len(), card);
            }
        }
    }

    #[test]
    fn generate_hand() {
        let deck = Deck::new();
        // let cards:Vec<Card> = Vec::new();
        let hand: Vec<&Card> = deck
            .cards
            .choose_multiple(&mut rand::thread_rng(), 3)
            .into_iter()
            .collect();
        // Chooses amount elements from the slice at random, without repetition, and in random order.
        for h in hand {
            println!("{:?}", h);
        }
    }

    #[test]
    pub fn check_for_trail() {}

    #[test]
    pub fn check_for_pair() {}

    #[test]
    pub fn check_for_flush() {} //all cards of the same suit

    //todo SAMAY
    #[test]
    pub fn check_for_sequence() {}

    #[test]
    pub fn check_for_pure_sequence() {}

    #[test] //not needed
    pub fn check_for_high_card() {
        // remaining else goes in this
    }
}
