#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

near delete near3.tranchinhwalletnear.testnet tranchinhwalletnear.testnet
near create-account near3.tranchinhwalletnear.testnet --masterAccount tranchinhwalletnear.testnet --initial-balance 10

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy near3.tranchinhwalletnear.testnet --wasmFile ./target/wasm32-unknown-unknown/release/hello_near.wasm
