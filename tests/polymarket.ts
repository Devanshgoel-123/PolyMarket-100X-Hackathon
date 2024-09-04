import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection, SystemProgram , SYSVAR_RENT_PUBKEY} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Polymarket } from "../target/types/polymarket";

describe("polymarket", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Polymarket as Program<Polymarket>;
  const connection = new Connection("https://api.testnet.solana.com", "confirmed");

  it("Initializes the central mint", async () => {
    const secretKey=new Uint8Array([
      126, 34, 89, 89, 143, 156, 34, 252, 218, 157, 26,
      178, 85, 41, 251, 54, 137, 16, 4, 220, 186, 87,
      190, 252, 111, 229, 72, 251, 93, 49, 216, 28, 137,
      63, 23, 215, 16, 56, 117, 83, 78, 252, 229, 60,
      56, 81, 204, 167, 148, 118, 128, 246, 176, 173, 121,
      106, 155, 57, 108, 235, 10, 158, 246, 174
       ]);
    const payer = Keypair.fromSecretKey(secretKey);
    const payerPublicKey = payer.publicKey;

    // Derive PDAs
    const [mintPDA, bumpMint] = await PublicKey.findProgramAddressSync(
      [Buffer.from("central_token_mint")],
      program.programId
    );
    const [vaultPDA, bumpVault] = await PublicKey.findProgramAddressSync(
      [Buffer.from("central_token_mint_vault")],
      program.programId
    );

    // Perform the method call
    try {
      const tx = await program.methods
        .createCentralMint()
        .accounts({
          tokenMint:mintPDA,
          payer: payerPublicKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([payer])
        .rpc();
      console.log("Transaction signature", tx);
    } catch (error) {
      console.error("Error executing transaction", error);
    }
  });
});