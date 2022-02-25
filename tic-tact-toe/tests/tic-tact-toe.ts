import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TicTactToe } from '../target/types/tic_tact_toe';
import { expect } from 'chai';

async function play(program, game, player,
  tile, expectedTurn, expectedGameState, expectedBoard) {
await program.rpc.play(tile, {
  accounts: {
    player: player.publicKey,
    game
  },
  signers: player instanceof (anchor.Wallet as any) ? [] : [player]
});

const gameState = await program.account.game.fetch(game);
expect(gameState.turn).to.equal(expectedTurn);
expect(gameState.state).to.eql(expectedGameState);
expect(gameState.board)
  .to
  .eql(expectedBoard);
}

describe('tic-tact-toe', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TicTactToe as Program<TicTactToe>;

  it('set up the game!', async () => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const playerOne = program.provider.wallet;
    const playerTwo = anchor.web3.Keypair.generate();
    await program.rpc.setupGame(playerTwo.publicKey, {
      accounts: {
        game: gameKeypair.publicKey,
        playerOne: playerOne.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [gameKeypair]
    });

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.turn).to.equal(1);
    expect(gameState.players)
      .to
      .eql([playerOne.publicKey, playerTwo.publicKey]);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board)
      .to
      .eql([[null,null,null],[null,null,null],[null,null,null]]);

  });

  it('player one wins', async() => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const playerOne = program.provider.wallet;
    const playerTwo = anchor.web3.Keypair.generate();
    await program.rpc.setupGame(playerTwo.publicKey, {
      accounts: {
        game: gameKeypair.publicKey,
        playerOne: playerOne.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [gameKeypair]
    });

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.turn).to.equal(1);
    expect(gameState.players)
      .to
      .eql([playerOne.publicKey, playerTwo.publicKey]);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board)
      .to
      .eql([[null,null,null],[null,null,null],[null,null,null]]);

    await play(
      program,
      gameKeypair.publicKey,
      playerOne,
      {row: 0, column: 0},
      2,
      { active: {}, },
      [
        [{x:{}},null,null],
        [null,null,null],
        [null,null,null]
      ]
    );
});
});
