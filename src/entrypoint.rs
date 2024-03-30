use solana_program::entrypoint;

use crate::instruction::process_instruction;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);
