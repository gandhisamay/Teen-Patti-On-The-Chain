# Teen Patti (based on the NEAR Protocol)
Teenpatti is an Indian origin card game. The objective of this game is to make the best 3 card hand as per the hand ranking and to maximize the pot (prize pool) before the showdown for you to win. A standard 52-card pack (without the jokers) is used in this game. Refer [Deltin](https://www.deltin.com/games/indian-flush-teen-patti#:~:text=In%20this%20game%2C%20the%20dealer,best%20hand%20wins%20the%20round.) for more detailed rules . 

# Ranking of the card hands from highest to lowest is

1. Trail (three of a kind) : 3 cards of the same rank. AAA is the best hand in the game.
2. Pure Sequence (Straight Flush): Three consecutive cards of the same suit.
3. Sequence (Straight): Three consecutive cards not all in the same suit.
4. Color (Flush): A flush of 3 cards in the same suit, with the highest card winning in the case of a draw.
5. Pair (two of a kind): A pair of 2 cards of the same rank. In case of a draw, the highest-ranking 3rd card will denote the winner.
6. High Card: When neither the dealer nor player has a pair, then the hand with the highest card wins.

## Getting started

To get it up and running up for your machine :

1. Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
2. Create a [NEAR](https://wallet.near.org/) account on the testnet or the mainnet network . 
3. Best practice is to create a sub account for eg. game.teenpatti.testnet and deploy your smart contract to that by using the deploy script . 
4. All scripts are stored in the scripts folder(make sure to replace the sub-account in the scripts using your account-id) 
 ![tree](https://user-images.githubusercontent.com/75066364/173606222-d22c643f-063a-477a-a2ce-2f8db8c5d750.jpg)




### HOW TO PLAY THE GAME 
There are 3 actions that a TeenPatti registered player can do using the NEAR CLI, namely:
1. RAISE - specify the raise amount,which should not be greater than the number of chips you have(i.e balance amount) in an argument provided to action to raise in the game 

 `near call <contract-account> play '{"action": "Raise{"amount": 45.0}" , "account_id": "player.testnet"}' --accountId <accountId>`
 
2. FOLD - when your cards are not good enough ,its better of to fold them using the following command . 

 `near call <contract-account> play '{"action": "Fold" , "account_id": "player.testnet"}' --accountId <accountId>`
 
3. SHOW - can only be called when 2 players are left in the game who have not yet folded their cards . After show the winner script runs and the winner is           announced . 

 `near call <contract-account> play '{"action": "Fold" , "account_id": "player.testnet"}' --accountId <accountId>`
 
 
 **Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
