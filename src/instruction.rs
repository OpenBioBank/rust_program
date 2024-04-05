//!Defines the program instructions and parameters

use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use solana_program::msg;

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

    Create,

    Test {
        id: u64,
        description: String,
        authorize: bool,
    },
}

impl Instruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Take the first byte as the variant to
        // determine which instruction to execute
        // let (&variant, rest) = input
        //     .split_first()
        //     .ok_or(ProgramError::InvalidInstructionData)?;
        // Use the temporary payload struct to deserialize
        let payload = InstructionPayload::try_from_slice(input)?;
        // Match the variant to determine which data struct is expected by
        // the function and return the TestStruct or an error
        Ok(match payload.methods_id {
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

            1 => Self::Create,

            2 => Self::Test {
                id: payload.id,
                description: payload.description,
                authorize: payload.authorize,
            },

            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
#[derive(BorshDeserialize)]
struct InstructionPayload {
    methods_id: u64,
    id: u64,
    description: String,
    owner: String,
    creator: String,
    authorize: bool,
    url: String,
    cid: String,
    is_mutable: bool,
}
