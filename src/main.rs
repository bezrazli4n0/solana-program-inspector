mod args;
mod error;
mod service;

use clap::Parser;
use error::Error;
use solana_client::rpc_client::RpcClient;

fn main() -> Result<(), Error> {
    let args = args::Args::parse();

    let client = RpcClient::new(args.url);

    let deployment_signature = service::get_deployment_signature(&client, &args.target_program_id)?;
    println!("[+] Deployment slot: {}", deployment_signature.slot);
    println!(
        "[+] Deployment block time: {:?}",
        deployment_signature.block_time
    );

    Ok(())
}
