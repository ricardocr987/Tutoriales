import * as anchor from "@project-serum/anchor";
import { Program, BN } from "@project-serum/anchor";
import { TheAlpha } from "../target/types/the_alpha";
import assert from "assert";
import * as helpers from "./helpers";
import { clusterApiUrl, Connection, Keypair, Transaction, SystemProgram } from "@solana/web3.js";
import {
  createInitializeMintInstruction,
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
  createMint,
  getMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import * as bs58 from "bs58";

const expect = require('chai').expect;

describe("the-alpha", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TheAlpha as Program<TheAlpha>;

  let authorAccount, authorAccountBump = null;
  let articleAccount, articleAccountBump = null;
  let readerAccount, readerAccountBump = null;
  
  [authorAccount, authorAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("author"), program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );

  [articleAccount, articleAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("article"),
        authorAccount.toBuffer(),
        new anchor.BN(0).toArrayLike(Buffer),
      ],
      program.programId
    );

  [readerAccount, readerAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("reader"), program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    
  const [firstArticleAccount, firstArticleAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("article"),
        authorAccount.toBuffer(),
        new anchor.BN(0).toArrayLike(Buffer),
      ],
      program.programId
    );

  const [secondArticleAccount, secondArticleAccountBump] =
    await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("article"),
        authorAccount.toBuffer(),
        new anchor.BN(1).toArrayLike(Buffer),
      ],
      program.programId
    );

  const [firstAuthorAccount, firstAuthorAccountBump] =
  await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("author"),
      authorAccount.toBuffer(),
    ],
    program.programId
  );

  const [secondAuthorAccount, secondAuthorAccountBump] =
  await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("article"),
      authorAccount.toBuffer(),
    ],
    program.programId
  );

  before(async () => {
    await helpers.requestAirdrop(program.provider.connection, program.provider.wallet.publicKey);
  });

  async function getAccountBalance(pubkey){
    let account = await program.provider.connection.getAccountInfo(pubkey);
  }

  function expectBalance(actual, expected, message, slack = 20000) {
    expect(actual, message).within(expected - slack, expected + slack);
  }

  // Token transfer variables
  let mint;
  let sender_token;
  let receiver_token;
  let receiver;
  

  it("Author account is initialized!", async () => {
    //const nft = anchor.web3.Keypair.generate();
    const name = 'Riki';
    const price = 255;
    const capacity = 255;
    const paid_or_free = true;
    const create_author_tx = await program.rpc.initializeAuthor(
      authorAccountBump,
      capacity,
      name,
      price,
      paid_or_free,
      //nft,
      {
      accounts: {
        authorAccount,
        user: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    console.log("Your transaction signature", create_author_tx);
    const authorState = await program.account.authorAccount.fetch(authorAccount);
    assert.equal(0, authorState.articleCount);
  });
/*
  it("Reader account is initialized!", async () => {
    const name = "Carlos";
    const vector_capacity = 255;

    const create_reader_tx = await program.rpc.initializeReader(
      readerAccountBump,
      vector_capacity,
      name,
      {
      accounts: {
        readerAccount,
        user: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    console.log("Your transaction signature", create_reader_tx);
    const readerState = await program.account.readerAccount.fetch(readerAccount);

    assert.equal(name, readerState.name);
    assert.equal([], readerState.timeSubVectorTuple);
  });*/

  it("Article account is initialized!", async () => {
    const paid_or_free = true; 
    const category = "nft";
    const create_article_tx = await program.rpc.initializeArticle(
      articleAccountBump,
      category,
      paid_or_free,
      {
      accounts: {
        articleAccount,
        authorAccount,
        user: program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    console.log("Your transaction signature", create_article_tx);
    const authorState = await program.account.authorAccount.fetch(authorAccount);
    const articleState = await program.account.articleAccount.fetch(articleAccount);

    assert.equal(1, articleState.articleId);
    assert.equal(1, authorState.articleCount);
    assert.equal(category, articleState.category);
    assert.equal(paid_or_free, authorState.articleCount);
  });

  it("Requires the correct signer to create a article", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    await helpers.requestAirdrop(program.provider.connection, newKeypair.publicKey);
    const newProvider = helpers.getProvider(program.provider.connection, newKeypair);
    const newProgram = helpers.getProgram(newProvider);
  
    let error;
  
    try {
      await newProgram.rpc.initializeArticle(secondArticleAccountBump, "nft", true, {
        accounts: {
          authorAccount,
          articleAccount: secondArticleAccount,
          user: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
    } catch (err) {
      error = err;
    } finally {
      assert.equal(error.message, "Signature verification failed");
    }
  });

  it("Updates a article", async () => {
    const paid_or_free = false; 
    const category = "crypto";

    await program.rpc.updateArticle(category, paid_or_free, {
      accounts: {
        authorAccount,
        articleAccount: firstArticleAccount,
        authority: program.provider.wallet.publicKey,
      },
    });
  
    const authorState = await program.account.authorAccount.fetch(authorAccount);
    const articleState = await program.account.articleAccount.fetch(articleAccount);
  
    assert.equal(1, articleState.articleId);
    assert.equal(1, authorState.articleCount);
    assert.equal(category, articleState.category);
    assert.equal(paid_or_free, articleState.paidOrFree);
  });
  
  it("Requires the correct signer to update a post", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    await helpers.requestAirdrop(program.provider.connection, newKeypair.publicKey);
    const newProvider = helpers.getProvider(program.provider.connection, newKeypair);
    const newProgram = helpers.getProgram(newProvider);
  
    let error;
  
    try {
      await newProgram.rpc.initializeArticle(firstArticleAccountBump, "nft", true, {
        accounts: {
          authorAccount,
          articleAccount: firstArticleAccount,
          user: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
    } catch (err) {
      error = err;
    } finally {
      assert.equal(error.message, "Signature verification failed");
    }
  });

  it("Updates a author", async () => {
    const paid_or_free = false; 
    const name = "Ricardocr987";
    const price_sub = 3;

    await program.rpc.updateAuthor(name, paid_or_free, price_sub, {
      accounts: {
        authorAccount,
        authority: program.provider.wallet.publicKey,
      },
    });
  
    const authorState = await program.account.authorAccount.fetch(authorAccount);
  
    assert.equal(1, authorState.articleCount);
    assert.equal(paid_or_free, authorState.paidOrFree);
    assert.equal(price_sub, authorState.priceSub);
  });
  
  it("Requires the correct signer to update the author", async () => {
    const newKeypair = anchor.web3.Keypair.generate();
    await helpers.requestAirdrop(program.provider.connection, newKeypair.publicKey);
    const newProvider = helpers.getProvider(program.provider.connection, newKeypair);
    const newProgram = helpers.getProgram(newProvider);
  
    let error;
  
    try {
      await newProgram.rpc.initializeAuthor(firstAuthorAccountBump, 255, "Ricardocr987", 3, false, {
        accounts: {
          authorAccount,
          user: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });
    } catch (err) {
      error = err;
    } finally {
      assert.equal(error.message, "Signature verification failed");
    }
  });
});
