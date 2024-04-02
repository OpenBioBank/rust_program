//!Defines the program instructions and parameters

use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Instruction {
    InitializeMintAccount {
        id: u64,
        description: String,
        owner: String,
        creator: String,
        authorize: bool,
        url: String,
        cid: String,
        is_mutable: bool,
    },

    FindCid,

    Create, 
}

impl Instruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Take the first byte as the variant to
        // determine which instruction to execute
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        // Use the temporary payload struct to deserialize
        let payload = InstructionPayload::try_from_slice(rest).unwrap();
        // Match the variant to determine which data struct is expected by
        // the function and return the TestStruct or an error
        Ok(match variant {
            0 => Self::InitializeMintAccount {
                id: payload.id,
                description: payload.description,
                owner: payload.owner,
                creator: payload.creator,
                authorize: payload.authorize,
                url: payload.url,
                cid: payload.cid,
                is_mutable: payload.is_mutable,
            },

            1 => Self::FindCid,
            2 => Self::Create,
            

            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
#[derive(BorshDeserialize)]
struct InstructionPayload {
    id: u64,
    description: String,
    owner: String,
    creator: String,
    authorize: bool,
    url: String,
    cid: String,
    is_mutable: bool,
}
