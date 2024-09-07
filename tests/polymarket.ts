import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID, getAccount, getTokenGroupState } from "@solana/spl-token";
import { Polymarket } from "../target/types/polymarket";

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
      await connection.requestAirdrop(payerPublicKey,10e9);
    }catch(err){
      console.log(err)
    }
    
    const balance2 = await connection.getBalance(payerPublicKey, { commitment: "finalized" });
    console.log("Payer Balance 2 :", balance2);
  };
  balanceFunction();

  it("Initializes the central mint", async () => {
    let vault=null;
    let mint=null;
    try {
      const [mintPDA, mintBump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("central_token_mint")],
        program.programId
      );
      const [vaultPDA, vaultBump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("central_token_mint_vault")],
        program.programId
      );
      vault=vaultPDA;
      mint=mintPDA;
      // console.log("Mint PDA:", mintPDA.toBase58());
      // console.log("Vault PDA:", vaultPDA.toBase58());
      const mintAccountInfo = await connection.getAccountInfo(mintPDA);
      const vaultAccountInfo = await connection.getAccountInfo(vaultPDA);
      
      const mintLamports = await connection.getMinimumBalanceForRentExemption(82); // 82 bytes for a token mint account
      const vaultLamports = await connection.getMinimumBalanceForRentExemption(165); // 165 bytes for a token vault account
  
      // Ensure accounts are funded
      if (mintAccountInfo === null) {
        await connection.requestAirdrop(payerPublicKey, mintLamports);
      }
      if (vaultAccountInfo === null) {
        await connection.requestAirdrop(payerPublicKey, vaultLamports);
      }try{
        const tx=await program.methods.createCentralMint().accounts({
          tokenMint: mintPDA,
          tokenVault: vaultPDA,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_2022_PROGRAM_ID
        }).signers([payer]).rpc();
       const txDetails=await connection.getTokenAccountBalance(vaultPDA);
       console.log(txDetails);
        // const balance=await getAccount(connection,vaultPDA,null,program.programId);
        // console.log(balance);
      }catch(err){
        console.log(err)
      }
       

    } catch (err) {
      console.error("Error initializing central mint:", err);
    }
  });
});
