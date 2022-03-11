import { Program } from "@project-serum/anchor";
import { ToDoList } from "../target/types/to_do_list";
const BN = require('bn.js');
const expect = require('chai').expect;
const anchor = require("@project-serum/anchor");
const { SystemProgram, LAMPORTS_PER_SOL } = anchor.web3;

describe("to-do-list", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  function expectBalance(actual, expected, message, slack=20000) {
    expect(actual, message).within(expected - slack, expected + slack)
  }

  async function createUser(airdropBalance) {
    airdropBalance = airdropBalance ?? 10 * LAMPORTS_PER_SOL;
    let user = anchor.web3.Keypair.generate();
    let sig = await program.provider.connection.requestAirdrop(user.publicKey, airdropBalance);
    await program.provider.connection.confirmTransaction(sig);

    let wallet = new anchor.Wallet(user);
    let userProvider = new anchor.Provider(program.provider.connection, wallet, program.provider.opts);

    return {
      key: user,
      wallet,
      provider: userProvider,
    };
  }

  function createUsers(numUsers) {
    let promises = [];
    for(let i = 0; i < numUsers; i++) {
      promises.push(createUser());
    }

    return Promise.all(promises);
  }

  async function getAccountBalance(pubkey) {
    let account = await program.provider.connection.getAccountInfo(pubkey);
    return account?.lamports ?? 0;
  }

  function programForUser(user) {
    return new anchor.Program(program.idl, program.programId, user.provider);
  }

  async function createList(owner, name, capacity=16) {
    const [listAccount, bump] = await anchor.web3.PublicKey.findProgramAddress([
      "todolist",
      owner.key.publicKey.toBytes(),
      name.slice(0, 32)
    ], program.programId);

    let program = programForUser(owner);
    await program.rpc.newList(name, capacity, bump, {
      accounts: {
        list: listAccount,
        user: owner.key.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });

    let list = await program.account.todoList.fetch(listAccount);
    return { publicKey: listAccount, data: list };
  }

  async function addItem({list, user, name, bounty}) {
    const itemAccount = anchor.web3.Keypair.generate();
    let program = programForUser(user);
    await program.rpc.add(list.data.name, name, new BN(bounty), {
      accounts: {
        list: list.publicKey,
        listOwner: list.data.listOwner,
        item: itemAccount.publicKey,
        user: user.key.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [
        user.key,
        itemAccount,
      ]
    });

    let [listData, itemData] = await Promise.all([
      program.account.todoList.fetch(list.publicKey),
      program.account.listItem.fetch(itemAccount.publicKey),
    ]);

    return {
      list: {
        publicKey: list.publicKey,
        data: listData,
      },
      item: {
        publicKey: itemAccount.publicKey,
        data: itemData,
      }
    };
  }

  async function cancelItem({ list, item, itemCreator, user }) {
    let program = programForUser(user);
    await program.rpc.cancel(list.data.name, {
      accounts: {
        list: list.publicKey,
        listOwner: list.data.listOwner,
        item: item.publicKey,
        itemCreator: itemCreator.key.publicKey,
        user: user.key.publicKey,
      }
    });

    let listData = await program.account.todoList.fetch(list.publicKey);
    return {
      list: {
        publicKey: list.publicKey,
        data: listData,
      }
    }
  }

  async function finishItem({ list, listOwner, item, user, expectAccountClosed }) {
    let program = programForUser(user);
    await program.rpc.finish(list.data.name, {
      accounts: {
        list: list.publicKey,
        listOwner: listOwner.key.publicKey,
        item: item.publicKey,
        user: user.key.publicKey,
      }
    });

    let [listData, itemData] = await Promise.all([
      program.account.todoList.fetch(list.publicKey),
      expectAccountClosed ? null : await program.account.listItem.fetch(item.publicKey),
    ]);

    return {
      list: {
        publicKey: list.publicKey,
        data: listData,
      },
      item: {
        publicKey: item.publicKey,
        data: itemData,
      }
    };
  }

  it("new list!", async () => {
    const owner = createUser();
    let list = await createList(Owner, 'a list');

    expect(list.data.listOwner.toString(), 'List owner is set').equals(owner.key.publicKey.toString());
    expect(list.data.name, 'List name is set').equals('A list');
  });
});