use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::env::{random_seed, random_seed_array};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};
use rand;
use rand::seq::SliceRandom;
use std::iter::Extend;

// 5 Ⓝ in yoctoNEAR
const INITIAL_BET: u128 = 5_000_000_000_000_000_000_000_000;

const SUITS: [&str; 4] = ["Spade", "Heart", "Club", "Diamond"];

const CARD_TYPES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];
const VALUES: [u16; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

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
    Flush, //3 cards of the same color,
    Pair,  //2 cards of the same rank
    HighCard,
    None,
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
pub struct Hand {
    cards: Vec<Card>,  //list of cards 
    hand_type: HandType,
}

impl Hand{
    pub fn max_card_val(&self) -> u16 {
        let mut largest = &self.cards[0].value;

        for card in &self.cards {
            if largest > &card.value {
                largest = &card.value;
            }
        }

        *largest
    }

    pub fn check_for_trail(&mut self) -> bool {
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

    pub fn check_for_pair(&mut self) -> bool {
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

    pub fn check_for_flush(&mut self) -> bool {
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

    pub fn set_players_hand_type(&mut self) {
        if self.check_for_trail() {
            self.hand_type = HandType::Trail;
        }
        //do this according to the hierarchy
        //
        // ..
        else if self.check_for_flush() {
            self.hand_type = HandType::Flush;
        } else if self.check_for_pair() {
            self.hand_type = HandType::Pair;
        } else {
            self.hand_type = HandType::HighCard;
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
    account_id: AccountId,
    hand: Hand,  //cards of the player and its type 
    name: String,
    betting_amount: f64, //tokens staked till now in the game
   
}

impl Player {
    // to compare the values of the decks 
    pub fn from(account_id: String, name: String, cards: Vec<Card>, betting_amount: f64) -> Self{
        Self{
            account_id: account_id.parse::<AccountId>().unwrap(),
            name,
            hand: Hand{
                cards,
                hand_type: HandType::None,
            },
            betting_amount,
        }
    }

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

    pub fn add_players(&mut self, input_players: Vec<AddPlayerInput>){
        for p in input_players{
            let player = Player::from(p.account_id, p.name, Vec::new(), 0.0);
            self.players.push(player);
        }
    }
}

//Helper structs
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddPlayerInput{
    pub account_id: String,
    pub name: String
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

    #[test] //three of a kind
    pub fn check_for_trail() {
        let cards = Deck::new().generate_hand();

        if cards[0].value == cards[1].value
            && cards[0].value == cards[2].value
            && cards[1].value == cards[2].value
        {
            assert!(true, "This is trail")
        } else {
            assert!(false, "not trail");
        }
    }

    #[test]
    pub fn check_for_pair() {
        let mut cards: Vec<Card> = Deck::new().generate_hand();

        if cards[0].value == cards[1].value && cards[1].value != cards[2].value {
            println!("true");
        } else if cards[1].value == cards[2].value && cards[1].value != cards[0].value {
            println!("true");
        } else if cards[0].value == cards[2].value && cards[0].value != cards[1].value {
            println!("true");
        } else {
            println!("false");
        }
    }

    #[test]
    pub fn check_for_flush() {
        let mut cards: Vec<Card> = Deck::new().generate_hand();

        // let card1 = Card {
        //     card_type: "J".to_owned(),
        //     suit: "Club".to_owned(),
        //     value: 11,
        // };
        // let card2 = Card {
        //     card_type: "3".to_owned(),
        //     suit: "Club".to_owned(),
        //     value: 11,
        // };
        // let card3 = Card {
        //     card_type: "4".to_owned(),
        //     suit: "Club".to_owned(),
        //     value: 11,
        // };

        // cards.push(card1);
        // cards.push(card2);
        // cards.push(card3);

        if cards[0].suit == cards[1].suit
            && cards[0].suit == cards[2].suit
            && cards[1].suit == cards[2].suit
        {
            assert!(true, "This is flush ")
        } else {
            assert!(false, "not flush");
        }
    } //all cards of the same suit

    //todo SAMAY
    #[test]
    pub fn check_for_sequence() {
        let mut cards: Vec<Card> = Deck::new().generate_hand();

        let mut cardsValue: Vec<i32> = Vec::new();

        for card in cards{
            cardsValue.push(card.value as i32);
        }

        cardsValue.sort();

        if (cardsValue.get(2).unwrap()-cardsValue.get(1).unwrap() == 1) && (cardsValue.get(1).unwrap()-cardsValue.get(0).unwrap() == 1){
            assert_eq!(true, "This is sequence");
        }
        else{
            assert_eq!(false, "This is not sequence");
        }
    }

    #[test]
    pub fn check_for_pure_sequence() {
        let mut cards: Vec<Card> = Deck::new().generate_hand();

        let mut cardsValue: Vec<i32> = Vec::new();

        for card in cards{
            cardsValue.push(card.value as i32);
        }

        cardsValue.sort();

        if (cardsValue.get(2).unwrap()-cardsValue.get(1).unwrap() == 1) && (cardsValue.get(1).unwrap()-cardsValue.get(0).unwrap() == 1){
            if (cards[0].suit == cards[1].suit) && (cards[1].suit == cards[2].suit){
                assert_eq!(true, "This is pure sequence");
            }
            else{
                assert_eq!(false, "This is not pure sequence");
            }
        }
        else{
            assert_eq!(false, "This is not pure sequence");
        }
    }

    #[test] //not needed
    pub fn check_for_high_card() {
        // remaining else goes in this
    }
}
