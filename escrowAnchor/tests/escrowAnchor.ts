import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { EscrowAnchor } from '../target/types/escrow_anchor';

describe('escrowAnchor', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.EscrowAnchor as Program<EscrowAnchor>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
