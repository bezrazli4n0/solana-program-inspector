use crate::Error;
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_program::bpf_loader_upgradeable;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use std::str::FromStr;

pub fn get_deployment_signature(
    client: &RpcClient,
    target_program_id: &Pubkey,
) -> Result<RpcConfirmedTransactionStatusWithSignature, Error> {
    let account = client.get_account(target_program_id)?;
    if !account.executable {
        return Err(Error::InvalidProgramAccount);
    }

    let (program_data, _) =
        Pubkey::find_program_address(&[target_program_id.as_ref()], &bpf_loader_upgradeable::id());

    let mut last_signature_data = None;
    let mut last_signature = None;
    loop {
        let config = GetConfirmedSignaturesForAddress2Config {
            before: last_signature,
            until: None,
            limit: None,
            commitment: Some(CommitmentConfig::finalized()),
        };

        let signatures = client.get_signatures_for_address_with_config(&program_data, config)?;
        if signatures.is_empty() {
            break;
        }

        let last_signature_data_temp = signatures.last().unwrap().clone();

        last_signature =
            Some(Signature::from_str(&last_signature_data_temp.signature.clone()).unwrap());
        last_signature_data = Some(last_signature_data_temp);
    }

    let last_signature_data = last_signature_data.unwrap();
    Ok(last_signature_data)
}
