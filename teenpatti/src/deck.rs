use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use crate::card::Card;
use crate::constants::*;

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Deck {
    pub cards: Vec<Card>,
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
        Deck { cards: cards }
    }

    // generates hand for one player
    pub fn generate_hand(&self) -> Vec<Card> {
        // let cards:Vec<Card> = Vec::new();
        // let hand: Vec<Card> = self
        //     .cards
        //     .choose_multiple(&mut rand::thread_rng(), 3)
        //     .cloned()
        //     .collect();

        let mut hand: Vec<Card> = Vec::new();

        let card1 = Card {
            card_type: "J".to_owned(),
            suit: "Club".to_owned(),
            value: 11,
        };
        let card2 = Card {
            card_type: "3".to_owned(),
            suit: "Club".to_owned(),
            value: 11,
        };
        let card3 = Card {
            card_type: "4".to_owned(),
            suit: "Club".to_owned(),
            value: 11,
        };

        hand.push(card1);
        hand.push(card2);
        hand.push(card3);
        // Chooses amount elements from the slice at random, without repetition, and in random order.
        hand
    }
}