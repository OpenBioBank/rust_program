### Environment and Tools Setup

Before starting the development of the NFT Minting Program, ensure your environment is setup with the necessary tools:

1.  **Install Rust**: Follow the instructions at the official [Rust install page](https://www.rust-lang.org/tools/install) 
1.  **Install Solana CLI**: Instructions for installing the Solana CLI can be found at the [Solana documentation](https://solana.com/zh/developers/guides/getstarted/setup-local-development).
1.  **Code Editor**: Install Visual Studio Code or any code editor of your choice. For Visual Studio Code, download from [Visual Studio Code website](https://code.visualstudio.com/).
1.  **Solana Tool Suite**: Ensure the Solana tools are installed by following the guide on the Solana CLI documentation.

### Project Directory Structure

Set up your project directory in VSCode or your preferred editor with the following structure:

-   `Cargo.toml` — The Rust project configuration file.
-   `src/lib.rs` — The entry point for the Solana program.
-   `src/processor.rs` — Contains the transaction processing logic.
-   `src/instruction.rs` — Defines the program instructions and parameters.
-   `src/error.rs` — Defines program-specific errors.
-   `src/state.rs` — Defines the program state and data structures.

### Compiling the Program

Compile the program using the following command in your terminal:

``` bash 
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program
```

### Deploying the Contract

Deploy your program to the Solana testnet or mainnet using the Solana CLI with the following command:

``` bash 
solana program deploy dist/program/nft_minting_program.so
```

Ensure that your Solana CLI is configured to the desired network (testnet or mainnet) before deploying the program. For more detailed steps on deploying, refer to the [Solana deployment documentation](https://www.solanazh.com/course/6-1).
