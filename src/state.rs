//!Defines the program state and data structures
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Nft {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub metadata_uri: String,
}

impl Sealed for Nft {}

impl IsInitialized for Nft {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Nft {
    const LEN: usize = 1 + 32 + 64; // Example size, adjust based on actual metadata_uri length

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut slice = dst;
        self.serialize(&mut slice).unwrap();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        let mut p = src;
        Ok(Self::deserialize(&mut p).unwrap())
    }
}
