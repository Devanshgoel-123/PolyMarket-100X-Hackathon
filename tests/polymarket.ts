import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Polymarket } from "../target/types/polymarket";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import {TOKEN_2022_PROGRAM_ID,createMint,getMint,getOrCreateAssociatedTokenAccount} from "@solana/spl-token";


describe("polymarket", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = new Connection("https://api.testnet.solana.com", "confirmed");
  const program = anchor.workspace.Polymarket as Program<Polymarket>;
  //test accounts

  it("Is initialized!", async () => {
    const tokenProgramId = TOKEN_2022_PROGRAM_ID;
    // Add your test here.
    const decimals=6;
    const secretKey=new Uint8Array([
      126,  34,  89,  89, 143, 156,  34, 252, 218, 157,  26,
      178,  85,  41, 251,  54, 137,  16,   4, 220, 186,  87,
      190, 252, 111, 229,  72, 251,  93,  49, 216,  28, 137,
       63,  23, 215,  16,  56, 117,  83,  78, 252, 229,  60,
       56,  81, 204, 167, 148, 118, 128, 246, 176, 173, 121,
      106, 155,  57, 108, 235,  10, 158, 246, 174
    ])
    try{
      const payer=Keypair.fromSecretKey(secretKey);
      console.log(payer.publicKey.toBase58());
      const publickey=new PublicKey(payer.publicKey.toBase58())
       
    }catch(err){
      console.log(err)
    }
    
  });
});
