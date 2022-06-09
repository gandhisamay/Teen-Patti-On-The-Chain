#!/bin/bash

echo ""
echo "deleting subaccount poker.teenpatti.testnet and sending the rest of NEAR to master account"

near delete game.teenpatti.testnet teenpatti.testnet
# deleting account also creates a tx for the masterAccount 

echo ""
echo "sub-account deleted"