mod programs;
#[cfg(test)] mod tests { 
    
    
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use solana_program::system_program;
    use solana_program::{
        pubkey::{self, Pubkey},
        system_instruction::transfer,
    };
    use solana_client::rpc_client::RpcClient;
    use std::str::FromStr;
    use crate::programs::turbin3_prereq::{self, CompleteArgs, UpdateArgs, WbaPrereqProgram};

    const RPC_URL: &str = "https://api.devnet.solana.com";


    #[test] 
    fn keygen() {

    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}",

    kp.pubkey().to_string()); 
    
    println!(""); println!("To save your wallet, copy and paste the following into a JSON file:"); 

    println!("{:?}", kp.to_bytes())

    }
     #[test] fn airdrop() {
        
          
            let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
          
            let client = RpcClient::new(RPC_URL);
        
           
            match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
                Ok(tx) => {
                    println!("Success! Check out your TX here:");
                    println!(
                        "https://explorer.solana.com/tx/{}?cluster=devnet",
                        tx.to_string()
                    );
                }
                Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
            }
        
        

     }
      #[test] fn transfer_sol() {
        let signer = read_keypair_file("wallet.json").expect("Couldn't find wallet file");
        // let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",digner,pubkey()]);
        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {github: b"ImmortalSul".to_vec() };
        let rpc_client = RpcClient::new(RPC_URL);
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let transaction =WbaPrereqProgram::complete(&[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()),&[&signer],

blockhash ); 
let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed
to send transaction");
    } 
    


    //You've generated a new Solana wallet: CU6H1kmJp5ZWUp3bKU9WVzjPvqPwuYDbbCQa52dFDXzi

// To save your wallet, copy and paste the following into a JSON file:
// [180, 42, 200, 91, 0, 89, 80, 236, 139, 134, 131, 85, 255, 47, 236, 208, 116, 21, 172, 166, 204, 160, 226, 231, 226, 176, 230, 67, 187, 131, 197, 113, 170, 97, 52, 39, 78, 164, 158, 196, 147, 236, 174, 145, 206, 129, 116, 196, 194, 37, 127, 60, 202, 50, 171, 229, 149, 145, 72, 102, 59, 161, 149, 203]
// test tests::keygen ... ok
}
    