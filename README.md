# Decentralized Teen Patti (NEAR Protocol)
Teenpatti is an Indian origin card game. The objective of this game is to make the best 3 card hand as per the hand ranking and to maximize the pot (prize pool) before the showdown for you to win. A standard 52-card pack (without the jokers) is used in this game.

## Problem Statement
Online card games like Rummy,TeenPatti are huge in India,attracting numerous masses and involving real money. But there are many problems in this sector namely :- 
### Accessibility and confidentiality

Businesses in the gaming industry offer crypto casino customers who use cryptocurrencies as an alternative to fiat money or a solid solution as their principal payment method. Players actively wagering on the platform can do so without providing identifying documentation. Additionally, there aren’t many restrictions on new participants entering the market with cryptocurrency. As a result, new players in the lottery system are exempted from the regulations that apply to traditional lotteries.

### Traceability

By utilizing the transparency feature built into blockchain technology, participants of the DeFi lottery platforms can access open records and examine all the historical information of trades. If any cyber attack occurs, participants can use the transparency function of a DeFi lottery platform to track and analyze it.

### Making Crypto Payments feasible

Participants can use their cryptos to subscribe for the use of the Defi lottery systems. Players on the DeFi lottery platform won’t have to be confined to their national borders thanks to the crypto payment system, which enables them to participate in events regulated outside their own countries. Since cryptocurrencies make payments and redemptions cheaper, utilizing DeFi lottery sites is significantly less expensive than conventional lottery platforms

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
To start the game, run the following command, after successful compilation of the files it will start by asking for player details and once the details are filled all the details will be pushed on the NEAR blockchain. 
```
cargo run --bin init
```
Now to further play game follow the actions given below.

### Game State
To to find information about 
1) All registered players 
2) Total staked tokens in the game by all players (i.e the pot)
3) Currently folded players 
4) Players currently in the game  

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
 Whenever the winner of game is found it is declared in the logs when either one of the 2 actions are performed :- 
 1. Only one player is left with unfolded cards and rest of the players jave folded theirs 
 2. When only 2 players remain with unfolded cards and a player calls "show" , then the winner is decided upon the hierarchy of their cards 

All the tokens staked in the game is then transferred to the winner instantly !(under development)
 
 ## Get more info

* [Rust Smart Contract Quick Start](https://docs.near.org/docs/develop/contracts/rust/intro)
* [Rust SDK Book](https://www.near-sdk.io/)
* [NEAR SDK Docs](https://www.near-sdk.io/)

## Contributors
Made with 💖 by Web3 enthusiasts, [Samay Gandhi](https://www.github.com/gandhisamay) and [Harsh Rathi](https://github.com/harshRathi2511)
