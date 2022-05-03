use solana_client::client_error::ClientError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("RPC client error.")]
    Rpc(ClientError),

    #[error("Invalid program account(data is not executable) error.")]
    InvalidProgramAccount,
}

impl From<ClientError> for Error {
    fn from(error: ClientError) -> Self {
        Error::Rpc(error)
    }
}
