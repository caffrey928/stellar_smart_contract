#!/bin/bash

# create identity if no identity exist
# soroban config identity generate --global telecom0

# give identity some testnet tokens
curl "http://localhost:8000/friendbot?addr=$(soroban config identity address telecom0)"

# deploy contract
echo "$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/arch.wasm --source telecom0 --network localhost_testnet)" \
 > .soroban/arch-local-id