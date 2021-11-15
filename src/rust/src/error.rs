use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MintError {
    #[error("Mint has concluded!")]
    NoneLeft,

    #[error("Authority key mis-match.")]
    AuthKeyFailure,

    #[error("This mint is NOT an NFT!")]
    InvalidMint,

    #[error("This token is empty!")]
    EmptyToken,

    #[error("The mint isn't out yet!")]
    Unavailable,

    #[error("You used all your presale tokens!")]
    Presale,

    #[error("The presale is just about to start!")]
    AlmostPresale
}

impl PrintProgramError for MintError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MintError> for ProgramError {
    fn from(e: MintError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MintError {
    fn type_of() -> &'static str {
        "Metadata Error"
    }
}

