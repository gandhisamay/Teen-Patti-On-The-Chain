use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use std::convert::From;
use std::convert::Into;

// 5 Ⓝ in yoctoNEAR
const INITIAL_BET: u128 = 5_000_000_000_000_000_000_000_000;

const INITIAL_COLLATERAL: u128 = 100_000_000_000_000_000_000_000_000;

const SUITS: [&str; 4] = ["Spade", "Heart", "Club", "Diamond"];

const CARD_TYPES: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];
const VALUES: [u16; 13] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

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

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum PlayerActions {
    Idle,
    Fold,
    Raise(f64),
    Show, //only when two players are remaining
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
pub enum HandType {
    // init field if required
    Trail, //3 of same rank
    PureSequence,
    Sequence,
    Flush, //3 cards of the same color,
    Pair,  //2 cards of the same rank
    HighCard,
}

impl From<HandType> for i32 {
    fn from(val: HandType) -> Self {
        match val {
            HandType::Trail => 1,
            HandType::PureSequence => 2,
            HandType::Sequence => 3,
            HandType::Flush => 4,
            HandType::Pair => 5,
            HandType::HighCard => 5,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Card {
    pub card_type: String,
    pub suit: String,
    pub value: u16,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Hand {
    cards: Vec<Card>, //list of cards
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

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
    pub account_id: AccountId,
    pub hand: Hand, //cards of the player and its type
    pub name: String,
    pub betting_amount: f64, //tokens staked till now in the game
    pub is_folded: bool,
    pub play_blind: bool,
    pub balance_amount: f64,
    // pub player_action: PlayerActions,
}

impl Player {
    pub fn from(
        account_id: String,
        name: String,
        cards: Vec<Card>,
        betting_amount: f64,
        is_folded: bool,
        play_blind: bool,
        balance_amount: f64,
        // player_action: PlayerActions,
    ) -> Self {
        Self {
            account_id: account_id
                .parse::<AccountId>()
                .expect("failed to parse account id"),
            name,
            hand: Hand { cards },
            betting_amount,
            is_folded,
            play_blind,
            balance_amount,
            // player_action,
        }
    }

    pub fn fold_cards(&mut self) {
        self.is_folded = true;
    }

    pub fn deposit_init_collateral(mut self) {
        // doesnt check if the account id is valid or not
        let player_account_id: AccountId = self.account_id;

        //   the master account for the teenpatti host
        let teenpatti_master_acc_id: AccountId = "teenpatti.testnet"
            .parse::<AccountId>()
            .expect("teenpatti.testnet account doesnt exist");

        // transfer the collateral amount
        // todo

        // hardcoded
    }

    // performs basic validation and returns the enum along with the amount, where the main logic is handled
    pub fn raise_amount(&mut self, raise_amount: f64) -> f64 {
        if raise_amount > self.balance_amount {
            env::panic_str("ERR: not enough balance")
        } else {
            // decrease from the balance and increase the betting amount
            self.balance_amount -= raise_amount;

            self.betting_amount += raise_amount;

            raise_amount
        }
    }

    pub fn show_cards(&mut self, players_remaining: usize) -> PlayerActions {
        if players_remaining == 2 {
            PlayerActions::Show
        } else {
            env::panic_str("ERR:cant use the show action when more than 2 players are remaining")
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub players: Vec<Player>,
    pub folded_players: Vec<Player>,
    pub tokens_staked: f64,
    pub unfolded_players: Vec<Player>,
    // pub current_turn_player: Vec<Player>,
}

#[near_bindgen]
impl Game {
    pub fn game_state(self) -> Game {
        self
    }
    pub fn get_players_data(self) -> Vec<Player> {
        self.players
    }

    pub fn get_player(self, account_id: AccountId) -> Player {
        for player in self.players {
            if player.account_id == account_id {
                // println!("{:?}",player);
                return player;
            }
        }

        let player1 = Player {
            account_id: "default.testnet"
                .parse::<AccountId>()
                .expect("failed to parse account id"),
            hand: Hand { cards: Vec::new() },
            name: String::from("dummy"),
            betting_amount: 0.0,
            is_folded: false,
            play_blind: false,
            balance_amount: 100.0,
        };

        player1
    }
    pub fn get_unfolded_players(&self) -> Vec<Player> {
        let mut unfolded_players: Vec<Player> = Vec::new();
        for player in &self.players {
            if player.is_folded == false {
                unfolded_players.push(player.clone())
            }
        }
        unfolded_players
    }

    pub fn get_player_by_account_id(
        &self,
        account_id: AccountId,
        player_list: &Vec<Player>,
    ) -> Player {
        for player in player_list {
            if player.account_id == account_id {
                env::log_str("found the account id");
                return player.clone();
            }
        }
        env::log_str("ERR: get_player_by_account_id: did not find player with account id");
        let player1 = Player {
            account_id: "dummy.testnet"
                .parse::<AccountId>()
                .expect("failed to parse account id"),
            hand: Hand { cards: Vec::new() },
            name: String::from("dummy"),
            betting_amount: 0.0,
            is_folded: false,
            play_blind: false,
            balance_amount: 100.0,
        };
        player1
    }

    pub fn print_unfolded(&self, unfolded_list: &Vec<Player>) {
        for player in unfolded_list {
            println!("{:?}", player);
        }
    }

    pub fn add_players(&mut self, input_players: Vec<AddPlayerInput>) {
        for p in input_players {
            let player = Player::from(p.account_id, p.name, p.cards, 0.0, false, false, 0.0);
            self.players.push(player);

            // set the unfolded list for the game
            self.unfolded_players = self.get_unfolded_players();

            // set the current turn for the player
            // self.current_turn_player = self.players[0].clone();
        }
    }

    pub fn play(&mut self, action: PlayerActions, account_id: AccountId) {
        env::log_str("getting whose turn to play");
        // let current_turn_player = &self.players[0];

        //set the unfolded players
        self.unfolded_players = self.get_unfolded_players();

        env::log_str("getting player making the request");
        let mut player = self.get_player_by_account_id(account_id, &self.players);

        // see if the player has its turn
        // if current_turn_player.account_id != player.account_id {
        //     env::panic_str("ERR: NOT YOUR TURN");
        // }

        // check if the current player has folded cards
        if player.is_folded == true {
            //works
            env::panic_str("ERR: you have folded already, can't play again");
        }

        // now check for how many players have folded cards
        if self.unfolded_players.len() <= 1 {
            //0,1,
            env::log_str("Congrats you are the winner");
        }

        match action {
            PlayerActions::Idle => env::log_str("ERR:PLAYER IDLE "),
            PlayerActions::Fold => {
                env::log_str("Folding");
                if let Some(index) = self.unfolded_players.iter().position(|x| *x == player) {
                    env::log_str("entered here :- folding the player");
                    // self.unfolded_players[index].is_folded = true; 
                    self.unfolded_players.remove(index);//this will change the game state of the unfolded players

                    if let Some(index) = self.players.iter().position(|x| *x == player) {
                        env::log_str("making self.players updated");
                        self.players[index].is_folded = true; //this will change the game state of the self.players
                    }

                    //push in the list of folded players
                    self.folded_players.push(player.clone());
                    // update the state in the folded players list too
                    if let Some(index) = self.folded_players.iter().position(|x| *x == player) {
                        self.folded_players[index].is_folded = true; //updated state
                    }

                    //set the current player to the new player
                    // if index == self.unfolded_players.len() {
                    //     //current turn is now of the first player
                    //     self.current_turn_player = self.unfolded_players[0].clone();
                    // } else {
                    //     //normal case where index player is now the current player as list popped left
                    //     self.current_turn_player = self.unfolded_players[index].clone();
                    // }
                } else {
                    env::panic_str("ERR: could not find the player in the list of unfolded people");
                }
            }
            PlayerActions::Raise(raise_amount) => {
                if let Some(index) = self.unfolded_players.iter().position(|x| *x == player) {
                    env::log_str("raising amount");
                    player.raise_amount(raise_amount); //just checks the balance of the player

                    // updating in the players list
                    if let Some(index) = self.players.iter().position(|x| *x == player) {
                        self.players[index].balance_amount -= raise_amount;
                        self.players[index].betting_amount += raise_amount;
                    }

                    //self.tokens_staked
                    self.tokens_staked += raise_amount;

                    // set the turn to the next player in the index if it exists
                    // if index < self.unfolded_players.len() - 1 {
                    //     //eg p1 raises but 3 players are there
                    //     self.current_turn_player = self.unfolded_players[index + 1].clone();
                    // } else {
                    //     //when index is equal to the len of the unfolded players
                    //     // index = self.unfolded_players.len()-1
                    //     // p3 raises when 3 players there
                    //     self.current_turn_player = self.unfolded_players[0].clone();
                    // }
                } else {
                    env::panic_str("ERR: could not find the player in the list of unfolded people");
                }
            }
            PlayerActions::Show => {
                env::log_str("showing action");
                if self.unfolded_players.len() == 2 {
                    // run winner script
                    env::log_str("should run winner script");
                } else {
                    env::panic_str(
                        "ERR:cant use the show action when more than 2 players are remaining",
                    )
                }
            }
        }

        // designate the current turn to other player in the unfolded list
    }

    pub fn find_winner(&mut self) -> &Player {
        //first find the players who have not folded
        let player1: &Player;
        let player2: &Player;

        let mut not_folded_players: Vec<&Player> = Vec::new();

        let player_iter = self.players.iter();

        for player in player_iter {
            if !player.is_folded {
                not_folded_players.push(player);
            }
        }

        player1 = not_folded_players.get(0).unwrap();
        player2 = not_folded_players.get(1).unwrap();

        let val1: i32 = player1.hand.get_players_hand_type().into();
        let val2: i32 = player2.hand.get_players_hand_type().into();

        if val1 < val2 {
            player1
        } else if val1 == val2 {
            let max_card1 = player1.hand.max_card_val();
            let max_card2 = player2.hand.max_card_val();

            if max_card1 >= max_card2 {
                player1
            } else {
                player2
            }
        } else {
            player2
        }
    }
}

//Helper structs
#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AddPlayerInput {
    pub account_id: String,
    pub name: String,
    pub cards: Vec<Card>
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::AccountId;

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

    pub fn get_unfolded_players(player_list: &Vec<Player>) -> Vec<Player> {
        let mut unfolded_players: Vec<Player> = Vec::new();
        for player in player_list {
            if player.is_folded == false {
                unfolded_players.push(player.clone())
            }
        }
        unfolded_players
    }

    pub fn get_player_by_account_id(account_id: AccountId, player_list: &Vec<Player>) -> Player {
        for player in player_list {
            if player.account_id == account_id {
                return player.clone();
            }
        }

        let player1 = Player {
            account_id: "dummy.testnet"
                .parse::<AccountId>()
                .expect("failed to parse account id"),
            hand: Hand { cards: Vec::new() },
            name: String::from("dummy"),
            betting_amount: 0.0,
            is_folded: false,
            play_blind: false,
            balance_amount: 100.0,
        };
        player1
    }

    pub fn print_unfolded(unfolded_list: &Vec<Player>) {
        for player in unfolded_list {
            println!("{:?}", player);
        }
    }

    #[test]
    pub fn get_player() {
        let mut player_list: Vec<Player> = Vec::new();
        let mut cards: Vec<Card> = Vec::new();

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

        cards.push(card1);
        cards.push(card2);
        cards.push(card3);

        let player1 = Player {
            account_id: "harshrathi2511.testnet"
                .parse::<AccountId>()
                .expect("failed to parse account id"),
            hand: Hand { cards: cards },
            name: String::from("Harsh Rathi"),
            betting_amount: 0.0,
            is_folded: false,
            play_blind: false,
            balance_amount: 100.0,
        };
        let player2 = Player {
            account_id: "samay200.testnet"
                .parse::<AccountId>()
                .expect("failed to parse account id"),
            hand: Hand { cards: Vec::new() },
            name: String::from("Samay Gandhi"),
            betting_amount: 0.0,
            is_folded: false,
            play_blind: false,
            balance_amount: 100.0,
        };
        player_list.push(player1);
        player_list.push(player2);

        let account_id = "harshrathi2511.testnet"
            .parse::<AccountId>()
            .expect("failed to parse account id");

        // println!("{:?}", player_data);
        let action = PlayerActions::Fold;
        let current_turn_player = &player_list[0];
        let mut unfolded_players = get_unfolded_players(&player_list);
        let mut player = get_player_by_account_id(account_id, &player_list);
        let mut tokens_staked = 0.0;

        if current_turn_player.account_id != player.account_id {
            env::panic_str("ERR: NOT YOUR TURN");
        }

        match action {
            PlayerActions::Idle => env::log_str("ERR:PLAYER IDLE "),
            PlayerActions::Fold => {
                print_unfolded(&unfolded_players);

                println!("FOLDING");
                if let Some(index) = unfolded_players.iter().position(|x| *x == player) {
                    println!("{}", index);
                    println!("entered here");
                    unfolded_players.remove(index);
                    // self.unfolded_players  equivalent
                    print_unfolded(&unfolded_players);
                } else {
                    env::panic_str("ERR: could not find the player in the list of unfolded people");
                }

                // fold the cards of the user
                player.fold_cards();
                // or
                player.is_folded = true;
            }
            PlayerActions::Raise(raise_amount) => {
                player.raise_amount(raise_amount);
                //self.tokens_staked
                tokens_staked += raise_amount;
            }
            PlayerActions::Show => {
                if unfolded_players.len() == 2 {
                    // run winner script
                } else {
                    env::panic_str(
                        "ERR:cant use the show action when more than 2 players are remaining",
                    )
                }
            }
        }
    }
}
