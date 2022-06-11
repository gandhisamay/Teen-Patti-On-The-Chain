use std::io::{self, Write};
use text_io::read;
use std::fs::File;


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

        if i+1 != num {
            string = format!("    {{\n      \"account_id\": \"{account_id}\",\n      \"name\": \"{name}\"\n    }},\n");
        }
        else{
            string = format!("    {{\n      \"account_id\": \"{account_id}\",\n      \"name\": \"{name}\"\n    }}\n");
        }
        println!();
        file.write_all(string.as_bytes()).unwrap();
    }

    file.write_all("  ]\n}' --accountId teen_patti2.testnet".as_bytes()).unwrap();

    Ok(())

}