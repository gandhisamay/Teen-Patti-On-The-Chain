use library::Contract;

fn main(){
    println!("Hello World!");
    let contract = Contract::start_game();
    dbg!(contract);
}