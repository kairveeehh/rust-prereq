#[cfg(test)]
mod tests {
    use solana_sdk::signature::{Keypair, Signer};
    use bs58;
    use std::io::{self, BufRead};

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
}
