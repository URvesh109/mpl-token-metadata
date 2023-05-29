#!/usr/bin/env bash
set -euo pipefail

solana airdrop -k program_ids/owner.json 1000

cargo build-bpf -- --ignore-rust-version

solana program deploy \
--program-id program_ids/token-metadata.json \
--keypair program_ids/owner.json \
target/deploy/mpl_token_metadata.so
