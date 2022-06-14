use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

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
                } else {
                    env::panic_str(
                        "ERR:cant use the show action when more than 2 players are remaining",
                    )
                }
            }
        }
    }
}