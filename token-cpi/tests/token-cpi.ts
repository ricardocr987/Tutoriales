import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TokenCpi } from '../target/types/token_cpi';
import { clusterApiUrl, Connection, Keypair, Transaction, SystemProgram } from "@solana/web3.js";
import {
  createInitializeMintInstruction,
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
  createMint,
  AccountLayout,
  getMinimumBalanceForRentExemptAccount,
  createInitializeAccountInstruction,
  createMintToCheckedInstruction,
} from "@solana/spl-token";

describe('token-cpi', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TokenCpi as Program<TokenCpi>;

  let mint;
  let sender_token;
  let receiver_token;
  let create_sender_token_tx;
  let create_receiver_token_tx

  it('setup mints and token accounts', async () => {
    // Add your test here.
    //const tx = await program.rpc.initialize({});
    //console.log("Your transaction signature", tx);
    mint = Keypair.generate();

    let create_mint_tx = new Transaction().add(
      // create mint account
      SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: mint.publicKey,
        space: MINT_SIZE,
        lamports: await getMinimumBalanceForRentExemptMint(program.provider.connection),
        programId: TOKEN_PROGRAM_ID,
      }),
      // init mint account
      createInitializeMintInstruction(
        mint.publicKey, // mint pubkey
        8, // decimals
        program.provider.wallet.publicKey, // mint authority
        program.provider.wallet.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
      )
    );
    //await program.provider.send(create_mint_tx, [mint]);
    console.log(`txhash: ${await program.provider.send(create_mint_tx, [mint])}`);

    sender_token = Keypair.generate();
    create_sender_token_tx = new Transaction().add(
      // create token account
      SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: sender_token.publicKey,
        space: AccountLayout.span,
        lamports: await getMinimumBalanceForRentExemptAccount(program.provider.connection),
        programId: TOKEN_PROGRAM_ID,
      }),
      // init mint account
      createInitializeAccountInstruction(
        sender_token.publicKey, // token account
        mint.publicKey, // mint
        program.provider.wallet.publicKey // owner of token account
      )
    );
    console.log(`txhash: ${await program.provider.send(create_sender_token_tx, [sender_token])}`);

    receiver_token = Keypair.generate();
    create_receiver_token_tx = new Transaction().add(
      // create token account
      SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: receiver_token.publicKey,
        space: AccountLayout.span,
        lamports: await getMinimumBalanceForRentExemptAccount(program.provider.connection),
        programId: TOKEN_PROGRAM_ID,
      }),
      // init mint account
      createInitializeAccountInstruction(
        receiver_token.publicKey, // token account
        mint.publicKey, // mint
        program.provider.wallet.publicKey // owner of token account
      )
    );
    console.log(`txhash: ${await program.provider.send(create_receiver_token_tx, [receiver_token])}`);

    let mint_tokens_tx = new Transaction().add(
      createMintToCheckedInstruction(
        mint.publicKey, // mint
        receiver_token.publicKey, // receiver (sholud be a token account)
        program.provider.wallet.publicKey, // mint authority
        1e8, // amount. if your decimals is 8, you mint 10^8 for 1 token.
        8 // decimals
        // [signer1, signer2 ...], // only multisig account will use
      )
    );
    console.log(`txhash: ${await program.provider.send(mint_tokens_tx)}`);
    console.log("token balance: ",await program.provider.connection.getTokenAccountBalance(sender_token.publicKey));
  });
});
