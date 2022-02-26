import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TicketingSystem } from "../target/types/ticketing_system";
import assert from "assert";

describe("ticketing-system", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TicketingSystem as Program<TicketingSystem>;
  const _ticketingSystem = anchor.web3.Keypair.generate();
  const tickets = [1111, 2222, 3333];

  it("Is initialized the ticketing system!", async () => {
    const ticketingSystem = _ticketingSystem;
    const tx = await program.rpc.initialize(tickets, {
      accounts: {
        ticketingSystem: ticketingSystem.publicKey,
        user: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [ticketingSystem],
    });
    const account = await program.account.ticketingSystem.fetch(
      ticketingSystem.publicKey
    );

    assert.ok(tickets.length === 3);
    assert.ok(
      account.tickets[0].owner.toBase58() ==
      ticketingSystem.publicKey.toBase58()
    );
  });
});
