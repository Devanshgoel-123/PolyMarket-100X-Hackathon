import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID, getAccount, getTokenGroupState } from "@solana/spl-token";
import { Polymarket } from "../target/types/polymarket";
import { assert } from "chai";

describe("polymarket", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Polymarket as Program<Polymarket>;
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");

  const secretKey = new Uint8Array([
    126, 34, 89, 89, 143, 156, 34, 252, 218, 157, 26,
    178, 85, 41, 251, 54, 137, 16, 4, 220, 186, 87,
    190, 252, 111, 229, 72, 251, 93, 49, 216, 28, 137,
    63, 23, 215, 16, 56, 117, 83, 78, 252, 229, 60,
    56, 81, 204, 167, 148, 118, 128, 246, 176, 173, 121,
    106, 155, 57, 108, 235, 10, 158, 246, 174,
  ]);
  const payer = Keypair.fromSecretKey(secretKey);
  const payerPublicKey = payer.publicKey;

  console.log("Payer Public Key:", payerPublicKey.toBase58());

  const balanceFunction = async () => {
    const balance1 = await connection.getBalance(payerPublicKey, { commitment: "finalized" });
    console.log("Payer Balance 1:", balance1);
    console.log("Finding airdrops");
    try{
      await connection.requestAirdrop(payerPublicKey,10000000);
    }catch(err){
      console.log(err)
    }
    
    const balance2 = await connection.getBalance(payerPublicKey, { commitment: "finalized" });
    console.log("Payer Balance 2 :", balance2);
  };
  // balanceFunction();
  let vault=null;
    let mint=null;
      const [mintPDA, mintBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("central_token_mint")],
        program.programId
      );
      const [vaultPDA, vaultBump] =PublicKey.findProgramAddressSync(
        [Buffer.from("central_token_mint_vault")],
        program.programId
      );
      vault=vaultPDA;
      mint=mintPDA;

  // it("Initializes the central mint", async () => {
    
  //     console.log("Mint PDA:", mintPDA.toBase58());
  //     console.log("Vault PDA:", vaultPDA.toBase58());
  //     const mintAccountInfo = await connection.getAccountInfo(mintPDA);
  //     const vaultAccountInfo = await connection.getAccountInfo(vaultPDA);
      
  //     // const mintLamports = await connection.getMinimumBalanceForRentExemption(82); // 82 bytes for a token mint account
  //     // const vaultLamports = await connection.getMinimumBalanceForRentExemption(165); // 165 bytes for a token vault account
  
  //     // // Ensure accounts are funded
  //     // if (mintAccountInfo === null) {
  //     //   await connection.requestAirdrop(payerPublicKey, mintLamports);
  //     // }
  //     // if (vaultAccountInfo === null) {
  //     //   await connection.requestAirdrop(payerPublicKey, vaultLamports);
  //     // }
  //     // if(mintAccountInfo===null){
  //     //   const tx=await program.methods.createCentralMint().accounts({
  //     //     tokenMint: mintPDA,
  //     //     tokenVault: vaultPDA,
  //     //     payer: payer.publicKey,
  //     //     systemProgram: SystemProgram.programId,
  //     //     rent: SYSVAR_RENT_PUBKEY,
  //     //     tokenProgram: TOKEN_2022_PROGRAM_ID
  //     //   }).signers([payer]).rpc();
  //     //  const txDetails=await connection.getTokenAccountBalance(vaultPDA);
  //     //  console.log(txDetails);
       
  //     // }
  //     const balance=await getAccount(connection,vaultPDA,null,TOKEN_2022_PROGRAM_ID);
  //     console.log(balance)
  //     console.log(balance.address.toBase58());
  //   });

  //  it("Initialize the Market",async()=>{
   
  //   const [marketAuthPDA,marketAuthBump]=await PublicKey.findProgramAddressSync(
  //     [Buffer.from("Sports"),Buffer.from("market_authority")],
  //     program.programId
  //   )
  
  //   const [marketStatePDA,marketStateBump]=await PublicKey.findProgramAddressSync(
  //     [Buffer.from("Sports"),Buffer.from("state")],
  //     program.programId
  //   )
  //   const [marketVaultPDA,marketVaultBump]=await PublicKey.findProgramAddressSync(
  //     [Buffer.from("Sports"),mintPDA.toBuffer()],
  //     program.programId
  //   )
  //   let marketinfo=await connection.getAccountInfo(marketAuthPDA);
  //   let marketVaultInfo=await connection.getAccountInfo(marketStatePDA);
  //   console.log("Fetching market info");
  //   console.log(marketinfo);
  //   console.log(marketVaultInfo);
  //   console.log(marketStatePDA.toBase58());
  //   console.log(marketAuthPDA.toBase58());
  //   console.log(marketVaultPDA.toBase58());
  //     try{
  //       const tx=await program.methods.createMarket("Sports").accountsStrict({
  //         marketAuthority:marketAuthPDA,
  //         marketState:marketStatePDA,
  //         tokenMint:mintPDA,
  //         marketVault:marketVaultPDA,
  //         payer:payer.publicKey,
  //         rent:SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //         tokenProgram:TOKEN_2022_PROGRAM_ID
  //       }).signers([payer]).rpc();
  //       console.log("Logging");
  //       console.log(tx);
  //       const marketAcct=await program.account.marketState.fetch(marketStatePDA);
  //       assert(marketAcct.authority.toBase58()===marketAuthPDA.toBase58());
  //       console.log("passed");
  //       assert(marketAcct.marketname==="Sports");
  //       console.log("passed");
  //       assert(marketAcct.balance.toNumber()===0);
  //       console.log("passed");
  //       assert(marketAcct.betArray.length===0);
  //       console.log("passed");
  //       assert(marketAcct.totalBets.toNumber()===0);
  //       console.log("passed");
  //       assert(marketAcct.tokenMint.toBase58()===mintPDA.toBase58());
  //       console.log("passed");
  //       // assert(marketAcct.creator===payerPublicKey);
  //       // console.log("passed");
  //       assert(marketAcct.marketAuthorityBump===marketAuthBump);
  //       console.log("passed");
  //       assert(marketAcct.marketStateBump===marketStateBump);
  //       console.log("passed");
  //     }catch(err){
  //       console.log(err);
  //     }
  //   const marketInfoAfterCreation=await connection.getAccountInfo(marketStatePDA);
  //     console.log(marketInfoAfterCreation.owner.toBase58())
  //  })
   it("Initialize the Bet",async()=>{
    const [betPDA,betBump]=PublicKey.findProgramAddressSync(
      [payerPublicKey.toBytes(),Buffer.from("Sports"),Buffer.from("Cricket")],
      program.programId
    );
    console.log(betPDA.toBase58());
    const [marketAuthPDA,marketAuthBump]=await PublicKey.findProgramAddressSync(
      [Buffer.from("Sports"),Buffer.from("market_authority")],
      program.programId
    )
console.log(marketAuthPDA.toBase58());
    const [marketStatePDA,marketStateBump]=await PublicKey.findProgramAddressSync(
      [Buffer.from("Sports"),Buffer.from("state")],
      program.programId
    )
  console.log(marketStatePDA.toBase58())
    const balance=connection.getBalance(payer.publicKey);
    try{
      const tx=await program.methods.createBet("Sports","Cricket","Ind Vs Pak").accounts({
        marketAuthority:marketAuthPDA,
        marketState:marketStatePDA,
        betState:betPDA,
        tokenMint:mintPDA,
        payer:payer.publicKey,
        rent:SYSVAR_RENT_PUBKEY,
        tokenProgram:TOKEN_2022_PROGRAM_ID,
        systemProgram:SystemProgram.programId
      }).signers([payer]).rpc();
      console.log(tx);
    }catch(err){
      console.log(err);
     } 
    console.log("Logging");
    // const betAccount=await connection.getAccountInfo(betPDA);
    const betAccountState=await program.account.bet.fetch(betPDA);
     console.log(betAccountState);

   })
   
});
