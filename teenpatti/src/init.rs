use std::io::{self, Write};
use text_io::read;
use std::fs::File;
use library::card::Card;
use library::deck::Deck;
use rand;
use rand::seq::SliceRandom;
use std::process::Command;

fn main() -> std::io::Result<()>{
    let mut file = File::create("add_players.sh")?;
    file.write_all("near call gaming.teen_patti2.testnet add_players '{\n \"input_players\":  [\n".as_bytes()).unwrap();
    print!("Welcome to Teen Patti \n");
    io::stdout().flush().unwrap();

    print!("Enter your no. of players: ");
    io::stdout().flush().unwrap();
    let num: i32 = read!();

    println!("Please enter your names!\n");

    for i in 0..num{
        println!("Player {}", i+1);
        println!("Testnet Account Name.");
        let account_id : String = read!();

        println!("Player Name.");
        let name : String = read!();

        let mut message = String::new();

        let cards = generate_cards();

        let card1_string = generate_card_string(cards.get(0).expect("Expected a card at index 0"), false);
        let card2_string = generate_card_string(cards.get(1).expect("Expected a card at index 1"), false);
        let card3_string = generate_card_string(cards.get(2).expect("Expected a card at index 2"), true);

        if i+1 != num {
            message = format!("    {{\n      \"account_id\": \"{account_id}\",\n      \"name\": \"{name}\",\n      \"cards\": [\n{card1_string}{card2_string}{card3_string}      ]\n    }},\n");
        }
        else{
            message = format!("    {{\n      \"account_id\": \"{account_id}\",\n      \"name\": \"{name}\",\n      \"cards\": [\n{card1_string}{card2_string}{card3_string}      ]\n    }}\n");
        }
        println!();
        file.write_all(message.as_bytes()).unwrap();
    }

    file.write_all("  ]\n}' --accountId teen_patti2.testnet".as_bytes()).unwrap();

    Command::new("bash").arg("add_players.sh").current_dir("/home/samaygandhi/Teen-Patti-On-The-Chain").spawn().unwrap();

    Ok(())

}

pub fn generate_card_string(card: &Card, last_card: bool) -> String{
    let card_type = &card.card_type;
    let suit  = &card.suit;
    let value = card.value;
    if last_card{
        let message = format!("        {{\n          \"card_type\": \"{card_type}\",\n          \"suit\": \"{suit}\",\n          \"value\": {value}    \n        }}\n");
        message
    }
    else{
        let message = format!("        {{\n          \"card_type\": \"{card_type}\",\n          \"suit\": \"{suit}\",\n          \"value\": {value}    \n        }},\n");
        message
    }
}

pub fn generate_cards() -> Vec<Card>{
    let deck = Deck::new();
    let hand: Vec<Card> = deck
    .cards
    .choose_multiple(&mut rand::thread_rng(), 3)
    .cloned()
    .collect();
// Chooses amount elements from the slice at random, without repetition, and in random order.
    hand
}