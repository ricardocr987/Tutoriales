import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Blog } from "../target/types/blog";
import assert from "assert";

describe("blog", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Blog as Program<Blog>;

  const [blogAccount, blogAccountBump] = 
    await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("blog"), 
      program.provider.wallet.publicKey.toBuffer()
    ],
      program.programId
    );

  const [firstPostAccount, firstPostAccountBump] = 
  await anchor.web3.PublicKey.findProgramAddress(
    [
      Buffer.from("post"),
      blogAccount.toBuffer(),
      new anchor.BN(0).toArrayLike(Buffer),
    ],
    program.programId,
  );

  it("Blog initialized with 0 entries!", async () => {
    await program.rpc.initializeBlog(blogAccountBump, {
      accounts: {
        blogAccount,
        user: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    })

    const blogState = await program.account.blogAccount.fetch(blogAccount);

    assert.equal(blogState.postCount, 0);
  });

  it("Post created and increment blog post counter", async () => {
    const title = "nft";
    const body = "ape";
    await program.rpc.createPost(
      firstPostAccountBump,
      title,
      body,{
        accounts: {
          blogAccount,
          postAccount: firstPostAccount,
          authority: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        }
      });

    const blogState = await program.account.blogAccount.fetch(blogAccount);
    const postState = await program.account.postAccount.fetch(firstPostAccount);


    assert.equal(title, postState.title);
    assert.equal(body, postState.body);
    assert.equal(0, postState.entry);
    assert.equal(1, blogState.postCount);
  });

  it("Update post", async () => {
    const title = "cripto";
    const body = "Solana";

    await program.rpc.updatePost(
      title,
      body, {
        accounts: {
          blogAccount,
          postAccount: firstPostAccount,
          authority: program.provider.wallet.publicKey,
        }
      })
    
      const blogState = await program.account.blogAccount.fetch(blogAccount);
      const postState = await program.account.postAccount.fetch(firstPostAccount);
      assert.equal(title, postState.title);
      assert.equal(body, postState.body);
      assert.equal(0, postState.entry);
      assert.equal(1, blogState.postCount);
  })
});
