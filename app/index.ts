import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import idl from "./artifacts/encode_sol_team3.json";
import * as dotenv from "dotenv";
import { EncodeSolTeam3 } from "./artifacts/encode_sol_team3";
import dayjs from "dayjs";
import {
  createTokenMint,
  findLaunchPoolAccount,
  findMintTokenAccount,
  findTreasurerAccount,
} from "./utils";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
dotenv.config();
const PROGRAM_ID = new PublicKey(
  "Eo9a3Zjn5HbGnL9wqkjDmajQ5EGzgaBbW77YhUZNVLo5"
);
const RPC_ENDPOINT = "https://api.devnet.solana.com";

(async () => {
  const connection = new web3.Connection(RPC_ENDPOINT, "confirmed");
  const creator_wallet = new Wallet(
    Keypair.fromSecretKey(
      Uint8Array.from(JSON.parse(process.env.CREATOR_WALLET))
    )
  );
  const alice_wallet = new Wallet(
    Keypair.fromSecretKey(Uint8Array.from(JSON.parse(process.env.ALICE_WALLET)))
  );
  const bob_wallet = new Wallet(
    Keypair.fromSecretKey(Uint8Array.from(JSON.parse(process.env.BOB_WALLET)))
  );

  const provider = new AnchorProvider(connection, creator_wallet, {
    preflightCommitment: "confirmed",
  });

  const program = new Program(
    idl as unknown as EncodeSolTeam3,
    PROGRAM_ID,
    provider
  );

  const mint = await createTokenMint(creator_wallet, creator_wallet.publicKey);

  await createNativeFairlaunchPool(program, creator_wallet, mint);
  await startLaunchPool(program, creator_wallet, mint);
})();

export async function createNativeFairlaunchPool(
  program: Program<EncodeSolTeam3>,
  creator: Wallet,
  mint: PublicKey,
  max = 100,
  min = 50,
  rate = new BN(50)
) {
  const unlock_date = new BN(dayjs().add(5, "s").unix());
  const pool_size = new BN(100 * LAMPORTS_PER_SOL);
  const minimum_token_amount = new BN(min * LAMPORTS_PER_SOL);
  const maximum_token_amount = new BN(max * LAMPORTS_PER_SOL);
  const [launch_pool] = findLaunchPoolAccount(
    creator.publicKey,
    mint,
    PROGRAM_ID
  );
  console.log(
    `launch_pool: ${launch_pool.toBase58()} creator: ${creator.publicKey.toBase58()} with mint: ${mint.toBase58()} creating ....`
  );
  console.log("--------------------------------------");

  const [treasurer] = findTreasurerAccount(launch_pool, mint, PROGRAM_ID);
  const treasury = await findMintTokenAccount(treasurer, mint);

  const tx = await program.methods
    .createNativePool(
      unlock_date,
      pool_size,
      minimum_token_amount,
      maximum_token_amount,
      rate,
      9
    )
    .accounts({
      launchPool: launch_pool,
      authority: creator.publicKey,
      tokenMint: mint,
      treasurer: treasurer,
      treasury: treasury,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([creator.payer])
    .rpc();

  console.log("Create a new launchpool in tx: ", "\n", tx);
  console.log("********************************");
}

export async function startLaunchPool(
  program: Program<EncodeSolTeam3>,
  creator: Wallet,
  mint: PublicKey
) {
  const [launch_pool] = findLaunchPoolAccount(
    creator.publicKey,
    mint,
    program.programId
  );
  const source_token_account = await findMintTokenAccount(
    creator.publicKey,
    mint
  );
  const [treasurer] = findTreasurerAccount(
    launch_pool,
    mint,
    program.programId
  );
  const treasury = await findMintTokenAccount(treasurer, mint);

  console.log(
    `launch_pool: ${launch_pool.toBase58()} creator: ${creator.publicKey.toBase58()} with mint: ${mint.toBase58()} starting ....`
  );
  console.log("--------------------------------------");
  const tx = await program.methods
    .startLaunchPool()
    .accounts({
      launchPool: launch_pool,
      tokenMint: mint,
      sourceTokenAccount: source_token_account,
      treasurer: treasurer,
      treasury: treasury,
      authority: creator.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
    })
    .signers([creator.payer])
    .rpc();
  console.log("Start launch pool in tx: ", "\n", tx);
  console.log("********************************");
}
