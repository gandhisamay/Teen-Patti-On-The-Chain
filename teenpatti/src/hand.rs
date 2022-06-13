use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use crate::card::Card;
use crate::handtype::HandType;
use crate::deck::Deck;

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Hand {
    pub cards: Vec<Card>, //list of cards
}

impl Hand {
    pub fn max_card_val(&self) -> u16 {
        let mut largest = &self.cards[0].value;

        for card in &self.cards {
            if largest > &card.value {
                largest = &card.value;
            }
        }

        *largest
    }

    pub fn check_for_trail(&self) -> bool {
        let cards = &self.cards;

        if cards[0].value == cards[1].value
            && cards[0].value == cards[2].value
            && cards[1].value == cards[2].value
        {
            true
        } else {
            false
        }
    }

    pub fn check_for_pair(&self) -> bool {
        let cards = &self.cards;

        if cards[0].value == cards[1].value && cards[1].value != cards[2].value {
            true
        } else if cards[1].value == cards[2].value && cards[1].value != cards[0].value {
            true
        } else if cards[0].value == cards[2].value && cards[0].value != cards[1].value {
            true
        } else {
            false
        }
    }

    pub fn check_for_flush(&self) -> bool {
        let cards = &self.cards;

        if cards[0].suit == cards[1].suit
            && cards[0].suit == cards[2].suit
            && cards[1].suit == cards[2].suit
        {
            true
        } else {
            false
        }
    }

    pub fn check_for_pure_sequence(&self) -> bool {
        let cards: Vec<Card> = Deck::new().generate_hand();

        let mut cards_value: Vec<i32> = Vec::new();

        for card in &cards {
            cards_value.push(card.value as i32);
        }

        cards_value.sort();

        if (cards_value.get(2).unwrap() - cards_value.get(1).unwrap() == 1)
            && (cards_value.get(1).unwrap() - cards_value.get(0).unwrap() == 1)
        {
            if (cards[0].suit == cards[1].suit) && (cards[1].suit == cards[2].suit) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn check_for_sequence(&self) -> bool {
        let cards: Vec<Card> = Deck::new().generate_hand();

        let mut cards_value: Vec<i32> = Vec::new();

        for card in cards {
            cards_value.push(card.value as i32);
        }

        cards_value.sort();

        if (cards_value.get(2).unwrap() - cards_value.get(1).unwrap() == 1)
            && (cards_value.get(1).unwrap() - cards_value.get(0).unwrap() == 1)
        {
            true
        } else {
            false
        }
    }

    pub fn get_players_hand_type(&self) -> HandType {
        if self.check_for_trail() {
            HandType::Trail
        } else if self.check_for_pure_sequence() {
            HandType::PureSequence
        } else if self.check_for_sequence() {
            HandType::Sequence
        } else if self.check_for_flush() {
            HandType::Flush
        } else if self.check_for_pair() {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }
}
