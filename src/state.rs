//!Defines the program state and data structures
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_pack::{IsInitialized, Pack, Sealed};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Data {
    cid: String,         //id can be used to find the off-chain data
    description: String, //RGB image
    //creator can not be deplayed
    url: String,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CreateMetadataAccountArgs {
    data: Data,
    is_mutable: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MetadataAccount {
    pub id: u32,
    pub description: String,
    pub owner: String,
    pub creator: String,
    pub authorize: bool,
    pub url: String,
    pub is_initialized: bool,
    pub cid: String,
    pub is_mutable: bool,
}

impl Sealed for MetadataAccount {}

impl IsInitialized for MetadataAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MetadataAccount {
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
