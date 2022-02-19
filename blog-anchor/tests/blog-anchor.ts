import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { BlogAnchor } from '../target/types/blog_anchor';

describe('blog-anchor', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.BlogAnchor as Program<BlogAnchor>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
