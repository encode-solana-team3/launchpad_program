import { Wallet, web3 } from "@coral-xyz/anchor";
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const connection = new web3.Connection(RPC_ENDPOINT, "confirmed");

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

export function findUserPoolAccount(
  user: PublicKey,
  pool: PublicKey,
  mint: PublicKey,
  programId: PublicKey
) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("userpool"),
      user.toBuffer(),
      pool.toBuffer(),
      mint.toBuffer(),
    ],
    programId
  );
}

export function delay(ms: number): Promise<void> {
  console.log(`delaying ${ms} ms ...`);
  return new Promise<void>((resolve) => setTimeout(resolve, ms));
}
