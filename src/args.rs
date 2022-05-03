use clap::{Parser, ValueHint};
use solana_sdk::pubkey::Pubkey;

/// CLI utility, that display additional information about deployed Solana program.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Cluster `URL`.
    #[clap(short, long, default_value = "https://api.mainnet-beta.solana.com", value_hint = ValueHint::Url)]
    pub url: String,

    /// `Pubkey` of target program for inspection.
    #[clap(short, long)]
    pub target_program_id: Pubkey,
}
