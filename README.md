# Rust Smart Contract Template

## Getting started

To get started with this template:

1. Click the "Use this template" button to create a new repo based on this template
2. Update line 2 of `Cargo.toml` with your project name
3. Update line 4 of `Cargo.toml` with your project author names
4. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
5. Begin writing your smart contract in `src/lib.rs`
6. Test the contract 

    `cargo test -- --nocapture`

8. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)

### HOW TO PLAY THE GAME 
There are 3 actions that a TeenPatti registered player can do using the NEAR CLI, namely:
1. RAISE - specify the raise amount,which should not be greater than the number of chips you have(i.e balance amount) in an argument provided to action to raise in the game 

 `near call <contract-account> play '{"action": "Raise{"amount": 45.0}" , "account_id": "player.testnet"}' --accountId <accountId>`
 
2. FOLD - when your cards are not good enough ,its better of to fold them using the following command . 

 `near call <contract-account> play '{"action": "Fold" , "account_id": "player.testnet"}' --accountId <accountId>`
 
3. SHOW - can only be called when 2 players are left in the game who have not yet folded their cards . After show the winner script runs and the winner is           announced . 

 `near call <contract-account> play '{"action": "Fold" , "account_id": "player.testnet"}' --accountId <accountId>`
