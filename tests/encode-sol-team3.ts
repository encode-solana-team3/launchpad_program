import * as anchor from "@coral-xyz/anchor";
import { BN, Program, web3 } from "@coral-xyz/anchor";
import dayjs from "dayjs";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { EncodeSolTeam3 } from "../target/types/encode_sol_team3";

// const RPC_ENDPOINT = "https://api.devnet.solana.com";
const RPC_ENDPOINT = "http://127.0.0.1:8899";
const connection = new web3.Connection(RPC_ENDPOINT, "confirmed");

describe("encode-sol-team3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EncodeSolTeam3 as Program<EncodeSolTeam3>;

  it("Is initialized!", async () => {
    const creator = (program.provider as anchor.AnchorProvider).wallet.publicKey;
    const mint = await createTokenMint((program.provider as anchor.AnchorProvider).wallet, creator, 1000000)
    const [launch_pool] = findLaunchPoolAccount(creator, mint, program.programId);
    const [treasurer] = findTreasurerAccount(
      launch_pool,
      mint,
      program.programId
    );
    const treasury = await findMintTokenAccount(treasurer, mint);

    const max = 100;
    const min = 50;
    const unlock_date = new BN(dayjs().add(5, "s").unix());
    const pool_size = new BN(1000 * LAMPORTS_PER_SOL);
    const minimum_token_amount = new BN(min * LAMPORTS_PER_SOL);
    const maximum_token_amount = new BN(max * LAMPORTS_PER_SOL);
    const rate = new BN(50);
    const tx = await program.methods.createNativePool(
      unlock_date,
      pool_size,
      minimum_token_amount,
      maximum_token_amount,
      rate,
      9
    ).accounts({
      launchPool: launch_pool,
      authority: creator,
      tokenMint: mint,
      treasurer: treasurer,
      treasury: treasury,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    }).rpc();

    console.log("Your transaction signature", tx);

  });
});


// launchpad-frontend/src/utils/account.ts 

export function findVestingPlanAccount(pool: PublicKey, programId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("vestingplan"), pool.toBuffer()],
    programId
  );
}

export function findLaunchPoolAccount(
  creator: PublicKey,
  mint: PublicKey,
  programId: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("launchpool"), creator.toBuffer(), mint.toBuffer()],
    programId
  );
}

export function findTreasurerAccount(
  pool: PublicKey,
  mint: PublicKey,
  programId: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("treasurer"), pool.toBuffer(), mint.toBuffer()],
    programId
  );
}

export async function findMintTokenAccount(owner: PublicKey, mint: PublicKey) {
  const token_account = await getAssociatedTokenAddressSync(mint, owner, true);
  return token_account;
}

export async function createTokenMint(
  creator: Wallet,
  to: PublicKey,
  amount = 1000000
) {
  const mint = await createMint(
    connection,
    creator.payer,
    creator.publicKey,
    null,
    9
  );

  console.log("Mint created: ", mint.toBase58());

  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    creator.payer,
    mint,
    to
  );

  await mintTo(
    connection,
    creator.payer,
    mint,
    tokenAccount.address,
    to,
    amount * LAMPORTS_PER_SOL
  );

  console.log(`Token minted to ${tokenAccount.address.toBase58()}`);

  return mint;
}

export function findVaultAccount(
  pool: PublicKey,
  creator: PublicKey,
  programId: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), pool.toBuffer(), creator.toBuffer()],
    programId
  );
}

export function findWhitelistAccount(pool: PublicKey, programId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("whitelist"), pool.toBuffer()],
    programId
  );
}
