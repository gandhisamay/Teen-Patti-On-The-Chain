use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};
use crate::hand::Hand;
use crate::card::Card;
use crate::player_actions::PlayerActions;

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