/*    https://lorisleiva.com/paginating-and-ordering-accounts-in-solana

                                        RPC Methods
                                        -----------

Let’s start by having a look at the RPC methods we’ll use to query the cluster.

getProgramAccounts. This RPC method allows us to fetch all accounts owned by a given program. For instance, 
                    if you’ve followed the series on how to create a Twitter dApp in Solana, this could fetch 
                    all the Tweet accounts of our program.

getAccountInfo. This RPC method allows us to get the account information and data from a given public key.

getMultipleAccounts. This RPC method does the same thing as getAccountInfo except that it retrieve multiple 
                    accounts from a provided list of public keys. This enables us to retrieve a bunch of accounts 
                    in only one API call to improve performances and avoid rate limiting. Note that the maximum number 
                    of accounts we can retrieve in one go using this method is 100. */

/*           
                                  RPC filtering and slicing
                                  -------------------------
Some of the RPC methods above support additional parameters to either filter or slice the accounts retrieved.

dataSlice. This parameter limits the data retrieved for each account. It expects an object containing an offset — 
        where the data should start — and a limit — how long the data should be. For example, providing { offset: 32, limit: 8 }
        will only retrieve 8 bytes of data starting at byte 32 for every account. This parameter is available on both getProgramAccounts 
        and getMultipleAccounts RPC methods.

dataSize. This parameter is a filter that only selects accounts whose data is of the given size in bytes. This filter is only available 
            on the getProgramAccounts RPC method. You can read more about this filter here.

memcmp. This parameter is a filter that only selects accounts such that their data matches the provided buffer at a given position. 
        This filter is only available on the getProgramAccounts RPC method.*/

import { Connection, clusterApiUrl, PublicKey } from '@solana/web3.js';
import { sha256 } from "js-sha256";
import bs58 from 'bs58';

const candyMachineV2Program = new PublicKey('cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ');

const connection = new Connection(clusterApiUrl('mainnet-beta'))

const candyMachineDiscriminator = Buffer.from(sha256.digest('account:CandyMachine')).slice(0, 8);

const accountsNotChecked = await connection.getProgramAccounts(candyMachineV2Program, {
    dataSlice: { offset: (8 + 32 + 32 + 33 + 8 + 4 + 6), length: 8 }, // Fetch the price only.
    // the 33 byte account is the publickey of the token mint, that has an extra bit (a boolean)
    // if it contains a pubkey that bit is 1 if not a 0, this is structure hasnt to be in that way
    // so there may be other programs with different structures to inform about that
    // for example you can store the pubkey state on PublickKey::default() and store on the token mint
    // variable 32 bits instead of 33, to identify that extra mint look next cons
    filters: [
        { memcmp: { offset: 0, bytes: bs58.encode(candyMachineDiscriminator) } }, // Ensure it's a CandyMachine account.
    ],
})

const accountsWithTokenMint = await connection.getProgramAccounts(candyMachineV2Program, {
    dataSlice: { offset: 8 + 32 + 32 + 33 + 8 + 4 + 6, length: 8 }, // Fetch the price only.
    filters: [
        { memcmp: { offset: 0, bytes: bs58.encode(candyMachineDiscriminator) } }, // Ensure it's a CandyMachine account.
        { memcmp: { offset: 8 + 32 + 32, bytes: bs58.encode((new BN(1, 'le')).toArray()) } }, // Ensure it has a token mint public key.
    ],
});

const accountsWithoutTokenMint = await connection.getProgramAccounts(candyMachineV2Program, {
    dataSlice: { offset: 8 + 32 + 32 + 1 + 8 + 4 + 6, length: 8 }, // Fetch the price only.
    filters: [
        { memcmp: { offset: 0, bytes: bs58.encode(candyMachineDiscriminator) } }, // Ensure it's a CandyMachine account.
        { memcmp: { offset: 8 + 32 + 32, bytes: bs58.encode((new BN(0, 'le')).toArray()) } }, // Ensure it doesn't have a token mint public key.
    ],
});

const accounts = [...accountsWithoutTokenMint, ...accountsWithTokenMint];

const accountsInTotal = accounts.length

const accountPublicKeys = accounts.map(account => account.pubkey)

const getPage = async (page, perPage) => {
    const paginatedPublicKeys = accountPublicKeys.slice(
        (page - 1) * perPage,
        page * perPage,
    );

    if (paginatedPublicKeys.length === 0) {
        return [];
    }
    const accountsWithData = await connection.getMultipleAccountsInfo(paginatedPublicKeys);

    return accountsWithData;
}

const perPage = 6;

const page1 = await getPage(1, perPage);
const page2 = await getPage(2, perPage);

const accountsWithPrice = accounts.map(({ pubkey, account }) => ({
    pubkey,
    price: new BN(account.data, 'le'),
}));

const sortedAccountsWithPrice = accountsWithPrice.sort((a, b) => b.price.cmp(a.price));

const accountPublicKeys = sortedAccountsWithPrice.map((account) => account.pubkey);

const top20ExpensiveCandyMachines = await getPage(1, 20);
