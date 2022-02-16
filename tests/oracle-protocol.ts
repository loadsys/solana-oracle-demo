import * as anchor from '@project-serum/anchor';
import assert from 'assert';
import { Program } from '@project-serum/anchor';
import { Provider } from '../target/types/provider';
import { Oracle } from '../target/types/oracle';

const { SystemProgram } = anchor.web3;

describe('oracle-protocol', () => {
  const env = anchor.Provider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(env);

  const providerProgram = anchor.workspace.Provider as Program<Provider>;
  const oracleProgram = anchor.workspace.Oracle as Program<Oracle>;


  let provider: anchor.web3.PublicKey = null;
  let oracle: anchor.web3.PublicKey = null;

  const user = anchor.web3.Keypair.generate();
  const unauthorizedUser = anchor.web3.Keypair.generate();

  const providerName = user.publicKey.toString().slice(0, 32);
  const oracleName = 'APPL/USD'

  let providerAccount = null

  before("Initialize program state", async () => {
    // Airdropping tokens to a payer.
    await env.connection.confirmTransaction(
      await env.connection.requestAirdrop(user.publicKey, 10000000000),
      "processed"
    );
    await env.connection.confirmTransaction(
      await env.connection.requestAirdrop(unauthorizedUser.publicKey, 10000000000),
      "processed"
    );
  });


  it('Is initialized!', async () => {
    const [_provider, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(providerName.slice(0, 32)),
      ],
      providerProgram.programId
    );
    provider = _provider;


    let state = null;
    try {
      state = await providerProgram.account.provider.fetch(provider);
    } catch(err) {
      console.log(err)
    }
    console.log(state, user.publicKey)

    if (state === null) {
      console.log('Creating provider')
      // Add your test here.
      const tx = await providerProgram.rpc.initialize(
        providerName,
        5,
        nonce,
        {
          accounts: {
            provider: provider,
            user: user.publicKey,
            systemProgram: SystemProgram.programId,
          },
          signers: [user],
        }
      );
      console.log("Your transaction signature", tx);

      state = await providerProgram.account.provider.fetch(provider)
      console.log(state)

    }
  });

  it('Create Oracle!', async () => {
    const [_oracle, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [
        provider.toBuffer(),
        Buffer.from(oracleName.slice(0, 32)),
      ],
      oracleProgram.programId
    );
    oracle = _oracle;


    let state = null;
    try {
      state = await oracleProgram.account.oracle.fetch(
        oracle
      );
    } catch(err) {
      console.log(err)
    }

    console.log('providerProgram.programId', providerProgram.programId.toString())

    if (state === null) {
      console.log('Creating oracle')
      // Add your test here.
      const tx = await oracleProgram.rpc.initialize(
        oracleName,
        [
          {
            name: 'price',
            value: "170.12",
          },
          {
            name: 'symbol',
            value: 'APPL',
          },
          {
            name: 'currency',
            value: 'USD',
          }
        ],
        nonce,
        {
          accounts: {
            oracle: oracle,
            oracleProvider: provider,
            user: user.publicKey,
            systemProgram: SystemProgram.programId,
          },
          signers: [user],
        }
      );
      console.log("Your transaction signature", tx);

      state = await oracleProgram.account.oracle.fetch(oracle)
      console.log('new state', state)

    }
  })

  it('Update Oracle!', async () => {
      // Add your test here.
      const tx = await oracleProgram.rpc.update(
        [
          {
            name: 'price',
            value: "179.12",
          },
          {
            name: 'symbol',
            value: 'APPL',
          },
          {
            name: 'currency',
            value: 'USD',
          }
        ],
        {
          accounts: {
            oracle: oracle,
            provider: provider,
            user: user.publicKey,
            systemProgram: SystemProgram.programId,
          },
          signers: [user],
        }
      );
      console.log("Your transaction signature", tx);

      const state = await oracleProgram.account.oracle.fetch(oracle)
      console.log('new state', state)

  })

  it('Unauthorized access!', async () => {
    let passed = false

    try {
      const tx = await oracleProgram.rpc.update(
        [
          {
            name: 'price',
            value: "175.12",
          },
          {
            name: 'symbol',
            value: 'APPL',
          },
          {
            name: 'currency',
            value: 'USD',
          }
        ],
        {
          accounts: {
            oracle: oracle,
            provider: provider,
            user: unauthorizedUser.publicKey,
            systemProgram: SystemProgram.programId,
          },
          signers: [unauthorizedUser],
        }
      );
    } catch (err) {
      passed = true
    }
    assert.ok(passed, 'Unauthorized user was able to update the record.')
  })
});
