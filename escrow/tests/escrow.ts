import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Escrow } from "../target/types/escrow";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Escrow as Program<Escrow>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
