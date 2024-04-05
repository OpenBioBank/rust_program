import {
    TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    createAssociatedTokenAccountInstruction,
  } from "@solana/spl-token";
  import {
    sendAndConfirmTransaction,
    Connection,
    SYSVAR_RENT_PUBKEY,
    SystemProgram,
  } from "@solana/web3.js";
  
  // Client
  class Assignable {
    constructor(properties) {
      Object.keys(properties).map((key) => {
        return (this[key] = properties[key]);
      });
    }
  }
  
  function intToBool(i: number) {
    if (i == 0) {
      return false;
    } else {
      return true;
    }
  }
  
  function boolToInt(t: boolean) {
    if (t) {
      return 1;
    } else {
      return 0;
    }
  }
  
  const boolMapper = {
    encode: boolToInt,
    decode: intToBool,
  };
  
  class InstructionData extends Assignable {
    toBuffer() {
      return Buffer.from(borsh.serialize(InstructionDataSchema, this));
    }
  }
  
  const InstructionDataSchema = new Map([
    [
      InstructionData,
      {
        kind: "struct",
        fields: [
          ["methods_id", "u64"],
          ["id", "u64"],
          ["description", "string"],
          ["owner", "string"],
          ["creator", "string"],
          ["authorize", "u8", boolMapper],
          ["url", "string"],
          ["cid", "string"],
          ["is_mutable", "u8", boolMapper],
        ],
      },
    ],
  ]);
  
  const blockhashInfo = await pg.connection.getLatestBlockhash();
  
  // Create transaction
  const tx = new web3.Transaction({
    ...blockhashInfo,
  });
  
  const nftMint = new InstructionData({
    methods_id: 0, // 2是test方法  0-1 调用报错
    id: 3,
    description: "hello world!----",
    owner: "Ee9tjcAXwDeHtVxKePgQos3YqxGo9sxExryCT8a1DFqe",
    creator: "Ee9tjcAXwDeHtVxKePgQos3YqxGo9sxExryCT8a1DFqe",
    authorize: true,
    url: "https://green-sad-canidae-844.mypinata.cloud/ipfs/QmcUgQvRjpgg1qhsfVNE497unLzMVkHbERQcPkCWdx5tyU/0.json",
    cid: "QmcUgQvRjp", //"gg1qhsfVNE497unLzMVkHbERQcPkCWdx5tyU",
    is_mutable: false,
  });
  
  const connection = new Connection("https://api.devnet.solana.com", "confirmed");
  const PROGRAM_ID = "CyGeFwXuqGHVV5HuzJ1iFMRqeL2YxU5CYTXyy5Cpbkg1";
  const pubkey = pg.wallet.keypair.publicKey;
  const cid = "QmcUgQvRjp";
  
  const [tokenMint] = web3.PublicKey.findProgramAddressSync(
    [pubkey.toBuffer(), Buffer.from(cid)],
    new web3.PublicKey(PROGRAM_ID)
  );
  
  const [mintAuth] = web3.PublicKey.findProgramAddressSync(
    [tokenMint.toBuffer()],
    new web3.PublicKey(PROGRAM_ID)
  );
  
  const [metadata] = web3.PublicKey.findProgramAddressSync(
    [tokenMint.toBuffer(), Buffer.from(cid)],
    new web3.PublicKey(PROGRAM_ID)
  );
  
  // const userAta = await getAssociatedTokenAddress(tokenMint, pubkey);
  // const ataAccount = await connection.getAccountInfo(userAta);
  
  // if (!ataAccount) {
  //   const ataInstruction = createAssociatedTokenAccountInstruction(
  //     pubkey,
  //     userAta,
  //     pubkey,
  //     tokenMint
  //   );
  
  //   tx.add(ataInstruction);
  // }
  
  // Add our hello world program instruction
  tx.add(
    new web3.TransactionInstruction({
      programId: new web3.PublicKey(
        "CyGeFwXuqGHVV5HuzJ1iFMRqeL2YxU5CYTXyy5Cpbkg1" // 部署程序的ID
      ),
      keys: [
        {
          pubkey: pubkey,
          isSigner: true,
          isWritable: false,
        },
        {
          pubkey: tokenMint,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: mintAuth,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: metadata,
          isSigner: false,
          isWritable: true,
        },
        // {
        //     pubkey: userAta,
        //     isSigner: false,
        //     isWritable: true,
        // },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: SYSVAR_RENT_PUBKEY,
          isSigner: false,
          isWritable: false,
        },
      ],
      data: nftMint.toBuffer(),
    })
  );
  
  // Sign transaction
  tx.sign(pg.wallet.keypair);
  
  // Send the transaction to the Solana cluster
  const txHash = await pg.connection.sendRawTransaction(tx.serialize());
  console.log(txHash);
  