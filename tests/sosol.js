const anchor = require("@project-serum/anchor");
const serumCmn = require("@project-serum/common");

const { TOKEN_PROGRAM_ID } = require("@solana/spl-token");

describe("sosol-tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Sosol;

  const MINT_TOKENS = 4200000000000000; // 42M with 8dp
  const MINT_DECIMALS = 8;

  let mint = null;
  let god = null;
  let consumerAcc = anchor.web3.Keypair.generate();
  let creatorAcc = anchor.web3.Keypair.generate();
  let storageAcc = anchor.web3.Keypair.generate();

  it("Sets up initial test state", async () => {
    const [_mint, _god] = await serumCmn.createMintAndVault(
      program.provider,
      new anchor.BN(MINT_TOKENS),
      undefined,
      MINT_DECIMALS
    );
    mint = _mint;
    god = _god;

    await program.provider.connection.requestAirdrop(consumerAcc.publicKey, 10000000),

    await serumCmn.createTokenAccount(
      program.provider,
      mint,
      consumerAcc.publicKey
    );

    await serumCmn.createTokenAccount(
      program.provider,
      mint,
      creatorAcc.publicKey
    );

    await serumCmn.createTokenAccount(
      program.provider,
      mint,
      storageAcc.publicKey
    );
  });

  it("Actions an interaction", async () => {
    const INTERACTION_FEE = 1000;
    const owner = program.provider.wallet.publicKey;

    console.log('*************', {
      from: consumerAcc.publicKey.toBase58(),
      to: creatorAcc.publicKey.toBase58(),
      toStorageAccount: storageAcc.publicKey.toBase58(),
      tokenProgram: TOKEN_PROGRAM_ID.toBase58(),
      programId: program.programId.toBase58(),
    });

    const myAccount = creatorAcc;

    await program.rpc.initialize(new anchor.BN(1234), {
      accounts: {
        myAccount: myAccount.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [myAccount],
      instructions: [await program.account.myAccount.createInstruction(myAccount)],
    });

    // await program.rpc.interaction(new anchor.BN(INTERACTION_FEE), {
    //   accounts: {
    //     from: program.provider.wallet.publicKey,
    //     to: creatorAcc.publicKey,
    //     toStorageAccount: storageAcc.publicKey,
    //     owner: program.provider.wallet.publicKey,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //     interactionFee: INTERACTION_FEE,
    //   },
    //   signers: [owner],
    // });

    // let _initializerTokenAccountA = await mintA.getAccountInfo(initializerTokenAccountA);

    // const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // console.log('****************', account);

    // Check it's state was initialized.
    // assert.ok(account.data.eq(new anchor.BN(1234)));

    // Check that the new owner is the PDA.
    // assert.ok(_initializerTokenAccountA.owner.equals(pda));

    // // Check that the values in the escrow account match what we expect.
    // assert.ok(_escrowAccount.initializerKey.equals(provider.wallet.publicKey));
    // assert.ok(_escrowAccount.initializerAmount.toNumber() == initializerAmount);
    // assert.ok(_escrowAccount.takerAmount.toNumber() == takerAmount);
    // assert.ok(
    //   _escrowAccount.initializerDepositTokenAccount.equals(initializerTokenAccountA)
    // );
    // assert.ok(
    //   _escrowAccount.initializerReceiveTokenAccount.equals(initializerTokenAccountB)
    // );
  });
});
