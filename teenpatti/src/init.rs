use std::io::{self, Write};
use text_io::read;
use std::fs::File;
use library::Card;
use library::Deck;
use rand;
use rand::seq::SliceRandom;

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

        let mut string = String::new();

        let cards = generate_cards();

        let card1_string = generate_card_string(cards.get(0).expect("Expected a card at index 0"), false);
        let card2_string = generate_card_string(cards.get(1).expect("Expected a card at index 1"), false);
        let card3_string = generate_card_string(cards.get(2).expect("Expected a card at index 2"), true);

        if i+1 != num {
            string = format!("    {{\n      \"account_id\": \"{account_id}\",\n      \"name\": \"{name}\",\n      \"cards\": [\n{card1_string}{card2_string}{card3_string}      ]\n    }},\n");
        }
        else{
            string = format!("    {{\n      \"account_id\": \"{account_id}\",\n      \"name\": \"{name}\",\n      \"cards\": [\n{card1_string}{card2_string}{card3_string}      ]\n    }}\n");
        }
        println!();
        file.write_all(string.as_bytes()).unwrap();
    }

    file.write_all("  ]\n}' --accountId teen_patti2.testnet".as_bytes()).unwrap();

    Ok(())

}

pub fn generate_card_string(card: &Card, last_card: bool) -> String{
    let card_type = &card.card_type;
    let suit  = &card.suit;
    let value = card.value;
    if last_card{
        let string = format!("        {{\n          \"card_type\": \"{card_type}\",\n          \"suit\": \"{suit}\",\n          \"value\": {value}    \n        }}\n");
        string
    }
    else{
        let string = format!("        {{\n          \"card_type\": \"{card_type}\",\n          \"suit\": \"{suit}\",\n          \"value\": {value}    \n        }},\n");
        string
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