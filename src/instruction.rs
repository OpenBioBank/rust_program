//!Defines the program instructions and parameters

use solana_program::pubkey::Pubkey;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Instruction {
    MintNft {
        //Data required to mint NFT
        metadata: String,
    },
    UpdateNftMetadata {
        // Update metadata with new values required
        new_metadata: String,
    },
    TransferOwnership {
        // Data required to transfer ownership, such as the new owner’s address
        new_owner: Pubkey,
    },
    // more....
}

impl Instruction {
    pub fn try_from_slice(_instruction_data: &[u8]) -> Instruction {
        todo!()
    }
}
