mod programs;
use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};



#[cfg(test)]

mod tests {
    use bs58;
    use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{self, read_keypair_file, Keypair, Signer},
        signer::{self, keypair},
        system_program,
        transaction::Transaction,
    };
    use std::io::{self, stdin, BufRead};
    use std::str::FromStr;
    
    use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram};
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

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("86gsWCEgo8VRr4rA3Bznh3jAArhL4TqrTBifxfyQfgzP").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get ");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance-fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
        //Transfer 0.001 SOL -  https://explorer.solana.com/tx/2Np3xkm5UdQHtjQbiJiFGoWDU1SYV1U6UxZQFBDoFrv7jCmnX8HZvrz3UAPHG4KPywYvaPA4ohVfBc41WtJceaof?cluster=devnet

        // complete wallet drain txid - https://explorer.solana.com/tx/4DnQi6hUfUxpGBEi3RfpF48RExhQbKRtrT9CWvrKce5rrtVBSW1BsJiCxLzD3XVSaDHWBDHR1j8b2qnGe5BQjQPG?cluster=devnet
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("turbine.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"kairveeehh".to_vec(),
        };
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Checkout your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
        //Txid -  https://explorer.solana.com/tx/5wMk9CiheUJVCnF2UZa8szSuCfeHusiZ4FcRfeVLk5zbGg9rxz98kFF4h5yRjtqhBVmbLNH2j1RgnRCYTK14c6KY?cluster=devnet
    }

}
