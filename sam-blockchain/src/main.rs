
use clap::{Parser, Subcommand};
use sam_blockchain::{blockchain::BlockChain, state, transaction::Transaction, wallet::{ Wallet}};
use ed25519_dalek::{SigningKey};

#[derive(Parser)]
#[command(name = "blockchain", about = "A simple Rust blockchain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewWallet,
    Balance {
        #[arg(short, long)]
        address: String,
    },
    Send {
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        privkey: String,
    },
    Mine {
        #[arg(short, long)]
        miner: String,
    },
    Show,
    Validate,
}


fn main() {
    let cli = Cli::parse();
    let mut chain: BlockChain = state::load("chain.json");

    match cli.command {
        Commands::NewWallet => {
            let wallet = Wallet::new();
            let privkey = hex::encode(wallet.signing_key.to_bytes());

            // build a simple JSON object with both keys
            let wallet_json = serde_json::json!({
                "public_key": wallet.public_key,
                "private_key": privkey
            });

            // save to a file named after the public key (first 8 chars)
            let filename = format!("wallet_{}.json", &wallet.public_key[..8]);
            std::fs::write(&filename, serde_json::to_string_pretty(&wallet_json).unwrap()).unwrap();

            println!("New wallet created:");
            println!("  Public key : {}", wallet.public_key);
            println!("  Private key : {}", privkey);
            println!("  Saved to   : {}", filename);
        }
        Commands::Balance { address } => {  
            let privkey_hex = state::load_wallet(&address)
                .expect("expected my wallet key");
            let signing_key = SigningKey::from_bytes(&hex::decode(&privkey_hex)
                .unwrap()
                .try_into()
                .unwrap());
            let wallet = Wallet::from_signing_key(signing_key);
            let balance = wallet.balance(&chain);
            println!("Balance for {}...: {:.2}", &address[..8], balance);
        }
        Commands::Send { from, to, amount, privkey } => {  
            // use provided privkey or try loading from wallet file
            let privkey_hex = if !privkey.is_empty() {
                privkey
            } else {
                state::load_wallet(&from).expect("no wallet found, pass --privkey manually")
            };

            let prvky_bytes: [u8; 32] = hex::decode(&privkey_hex)
                .unwrap()
                .try_into()
                .unwrap();
            let signing_key = SigningKey::from_bytes(&prvky_bytes);
            let wallet = Wallet::from_signing_key(signing_key);

            let tr = Transaction::new(
                    &wallet.public_key, &to, amount);
            chain.submit_transaction(tr);
            
            println!("transaction submitted {}", chain.mempool_size());
        }
        Commands::Mine { miner } => {
            if chain.mempool_size() == 0 {
                println!("No pending transactions to mine.");
            } else {
                println!("Pending transactions: {}", chain.mempool_size());
                chain.mine_pending_trans(&miner);
                println!("Mined block added to chain.");
            }
        }
        Commands::Show => { 
            println!("{}", chain);
        }
        Commands::Validate => {
            if chain.is_valid() {
                println!("Chain is valid ✓");
            } else {
                println!("Chain is invalid ✗");
            }
        }
    }

    state::save(&chain, "chain.json");
}

