#!/bin/bash 

near call game.teenpatti.testnet start_game --accountId harshrathi2511.testnet

near call game.teenpatti.testnet register_player '{
    "account_name": "harshrathi2511.testnet",
    "name" : "Harsh Rathi",
    ...
    
}' --accountId harshrathi2511.testnet 

