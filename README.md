# Decentralized Teen Patti (NEAR Protocol)
Teenpatti is an Indian origin card game. The objective of this game is to make the best 3 card hand as per the hand ranking and to maximize the pot (prize pool) before the showdown for you to win. A standard 52-card pack (without the jokers) is used in this game.

## Rules of the game

### Ranking of the card hands from highest to lowest is

1) Trail (three of a kind) : 3 cards of the same rank. AAA is the best hand in the game.
2) Pure Sequence (Straight Flush): Three consecutive cards of the same suit.
3) Sequence (Straight): Three consecutive cards not all in the same suit.
4) Color (Flush): A flush of 3 cards in the same suit, with the highest card winning in the case of a draw.
5) Pair (two of a kind): A pair of 2 cards of the same rank. In case of a draw, the highest-ranking 3rd card will denote the winner.
6) High Card: When neither the dealer nor player has a pair, then the hand with the highest card wins.

For more detailed rules refer [Deltin](https://www.deltin.com/games/indian-flush-teen-patti#:~:text=In%20this%20game%2C%20the%20dealer,best%20hand%20wins%20the%20round.). 

## Installation

To get it up and running up for your machine :

1) Set up the [prerequisites](https://github.com/near/near-sdk-rs#pre-requisites)
2) Create a [NEAR](https://wallet.near.org/) account on the testnet or the mainnet network . 
3) Best practice is to create a sub account for eg. game.teenpatti.testnet and deploy your smart contract to that by using the deploy script.
![tree](https://user-images.githubusercontent.com/75066364/173606222-d22c643f-063a-477a-a2ce-2f8db8c5d750.jpg)
4) Clone the project and cd into the folder. 
 
Everywhere replace,<br> 
    `<accountId>` by your NEAR testnet account <br>
    `<contract-account>` by the NEAR sub-account created where the game is deployed.


## Game Play

### Start game
To start the game, run the following command, after successful compilation of the files, the game will start and it will start with asking for playing details and once the details are filled all the player data will be pushed on the NEAR blockchain. 
```
cargo run --bin init
```
### Game State
To to find information like 
1) All registered players info
2) Total staked tokens info
3) Currently folded players info
4) Players currently in the game info 

**Run**
```
near view <contract-account> game_state
```

**Output**
```
View call: gaming.teen_patti2.testnet.game_state()
{
  players: [
    {
      account_id: 'samay200.testnet',
      hand: {
        cards: [
          { card_type: 'J', suit: 'Heart', value: 11 },
          { card_type: '9', suit: 'Spade', value: 9 },
          { card_type: 'J', suit: 'Diamond', value: 11 }
        ]
      },
      name: 'Samay',
      betting_amount: 80,
      is_folded: false,
      play_blind: false,
      balance_amount: 420
    },
    {
      account_id: 'harshrathi2511.testnet',
      hand: {
        cards: [
          { card_type: '8', suit: 'Spade', value: 8 },
          { card_type: 'Q', suit: 'Diamond', value: 12 },
          { card_type: '10', suit: 'Heart', value: 10 }
        ]
      },
      name: 'Harsh',
      betting_amount: 0,
      is_folded: true,
      play_blind: false,
      balance_amount: 500
    },
    {
      account_id: 'teen_patti2.testnet',
      hand: {
        cards: [
          { card_type: '7', suit: 'Club', value: 7 },
          { card_type: '9', suit: 'Club', value: 9 },
          { card_type: '3', suit: 'Club', value: 3 }
        ]
      },
      name: 'TeenPatti',
      betting_amount: 60,
      is_folded: false,
      play_blind: false,
      balance_amount: 580
    }
  ],
  folded_players: [
    {
      account_id: 'harshrathi2511.testnet',
      hand: {
        cards: [
          { card_type: '8', suit: 'Spade', value: 8 },
          { card_type: 'Q', suit: 'Diamond', value: 12 },
          { card_type: '10', suit: 'Heart', value: 10 }
        ]
      },
      name: 'Harsh',
      betting_amount: 0,
      is_folded: true,
      play_blind: false,
      balance_amount: 500
    }
  ],
  tokens_staked: 140,
  unfolded_players: [
    {
      account_id: 'samay200.testnet',
      hand: {
        cards: [
          { card_type: 'J', suit: 'Heart', value: 11 },
          { card_type: '9', suit: 'Spade', value: 9 },
          { card_type: 'J', suit: 'Diamond', value: 11 }
        ]
      },
      name: 'Samay',
      betting_amount: 80,
      is_folded: false,
      play_blind: false,
      balance_amount: 420
    },
    {
      account_id: 'teen_patti2.testnet',
      hand: {
        cards: [
          { card_type: '7', suit: 'Club', value: 7 },
          { card_type: '9', suit: 'Club', value: 9 },
          { card_type: '3', suit: 'Club', value: 3 }
        ]
      },
      name: 'TeenPatti',
      betting_amount: 60,
      is_folded: false,
      play_blind: false,
      balance_amount: 580
    }
  ]
} 
```

### Play Actions

There are 3 actions that a TeenPatti registered player can be done using the NEAR CLI, namely:
1) **RAISE** - specify the raise amount,which should not be greater than the number of chips you have(i.e balance amount) in an argument provided to action to raise in the game 

```
near call <contract-account> play 
    '{
       "action": "Raise{"amount": 45.0}", 
       "account_id": <accountId>
     }' 
--accountId <accountId>
```
 
2) **FOLD** - when your cards are not good enough ,its better of to fold them using the following command . 

 ```
 near call <contract-account> play 
     '{
        "action": "Fold", 
        "account_id": <accountId>
      }' 
--accountId <accountId>
 ```
 
3) **SHOW** - can only be called when 2 players are left in the game who have not yet folded their cards . After show the winner script runs and the winner is           announced . 

 ```
 near call <contract-account> play 
     '{
        "action": "Show", 
        "account_id": <accountId>
      }' 
--accountId <accountId>
 ```
 
 ### Game Winner
 Whenever the winner of game is found it is declared using the logs when the last feasible action is performed!
 
 ## Get more info

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
* [NEAR SDK Docs](https://www.near-sdk.io/)
