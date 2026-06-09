

# 1. create two wallets
cargo run -- new-wallet
> New wallet created:
>   Public key : a3f9c2f1...
>   Private key: 7b4e1d2c...
>   Saved to   : wallet_a3f9c2f1.json

cargo run -- new-wallet
> New wallet created:
>   Public key : 9f3b1a4d...
>   Private key: 4c8d2e1f...
>   Saved to   : wallet_9f3b1a4d.json

# 2. check balances (both empty, no transactions yet)
cargo run -- balance --address a3f9c2f1...
> Balance for a3f9c2f1...: 0.00

cargo run -- balance --address 9f3b1a4d...
> Balance for 9f3b1a4d...: 0.00

# 3. mine a block to give alice the mining reward
cargo run -- mine --miner a3f9c2f1...
> No pending transactions to mine.

# 4. submit some transactions
cargo run -- send --from a3f9c2f1... --to 9f3b1a4d... --amount 10.5 --privkey 7b4e1d...
> Transaction submitted. Pending: 1

cargo run -- send --from a3f9c2f1... --to 9f3b1a4d... --amount 5.0
> Transaction submitted. Pending: 2

# 5. mine pending transactions
cargo run -- mine --miner a3f9c2f1...
> Pending transactions: 2
> Mining block #1... done in 43ms (nonce=38291)
> Block added to chain.

# 6. check balances after mining
cargo run -- balance --address a3f9c2f1...
> Balance for a3f9c2f1...: 34.50   (50 reward - 10.5 - 5.0)

cargo run -- balance --address 9f3b1a4d...
> Balance for 9f3b1a4d...: 15.50   (10.5 + 5.0)

# 7. validate the chain
cargo run -- validate
> Chain is valid ✓

# 8. show the full chain
cargo run -- show
> ==================== Blockchain ====================
> Block #0
>   Timestamp : 1715000000
>   Nonce     : 0
>   Prev Hash : 0000...0000
>   Hash      : 3a7bd3...
>   Transactions: (none)
> ----------------------------------------------------
> Block #1
>   Timestamp : 1715000001
>   Nonce     : 38291
>   Prev Hash : 3a7bd3...
>   Hash

