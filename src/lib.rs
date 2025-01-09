#[cfg(test)]
mod tests {
    use solana_sdk::signature::{Keypair, Signer};
    use bs58;
    use solana_client::rpc_client::RpcClient; 
    use solana_sdk::signer::keypair::read_keypair_file;
    use std::io;
    use std::io::BufRead;
  
    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana Wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
        //generated wallet - CkutiP1SHQKB1EaKt9zyaNxSQEHfKss9rAwpB6MUyGk3
    }
    
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58: ");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is: ");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here: ");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
      // txid -   https://explorer.solana.com/tx/3AfnQLa3JwYWNwHqMatjwRFHouY8ty34zhSuhradb1Aa3AtDzdQbQcimS5wHSQXxVgtUE5bpnaDvbSs8PUVrfVHz?cluster=devnet
    }

}
