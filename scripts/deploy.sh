#!/bin/bash
set -e
chmod +x scripts/deploy.sh 

# echo "enter masterAccount..."
# read masterAccount
# echo "master account is $masterAccount"

echo ""
echo "building wasm file to be deployed........."
echo ""
# giving permissions to the build script to execute  
./scripts/build.sh

echo ""
echo "creating a new testnet subaccount of teenpatti.testnet"
echo ""
# should ideally create a random testnet account and then deploy to it  
near create-account game.teenpatti.testnet --masterAccount teenpatti.testnet
echo ""

echo "viewing the state of the subaccount game.teenpatti.testnet"
echo ""

echo "if the code hash has all ones :- NO CONTRACT DEPLOYED YET "
echo ""

near state game.teenpatti.testnet
echo ""

echo "deploying the contract (wasm file) to the subaccount ...using batch actions to #[init] "
echo ""

# using batch actions to instantiate the function 
near deploy game.teenpatti.testnet --wasmFile res/library.wasm \
# #  --initFunction 'new'  \
# #  --initArgs '{"owner_id": "game.teenpatti.testnet"}'
 
echo ""
echo ""

echo " checking if the code hashes are all ones.........."
echo ""
echo ""

near state game.teenpatti.testnet 
echo ""
echo ""
echo ""

echo "contract deployed successfully    "
echo ""