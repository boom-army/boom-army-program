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
  let creatorAcc = anchor.web3.Keypair.generate();
  let creatorTokenAcc = null;
  let storageAcc = anchor.web3.Keypair.generate();
  let storageTokenAcc = null;

  it("Sets up initial test state", async () => {
    const [_mint, _god] = await serumCmn.createMintAndVault(
      program.provider,
      new anchor.BN(MINT_TOKENS),
      undefined,
      MINT_DECIMALS
    );
    mint = _mint;
    god = _god;

    creatorTokenAcc =await serumCmn.createTokenAccount(
      program.provider,
      mint,
      creatorAcc.publicKey
    );

    storageTokenAcc = await serumCmn.createTokenAccount(
      program.provider,
      mint,
      storageAcc.publicKey
    );
  });

  it("Actions an interaction", async () => {
    const INTERACTION_FEE = 10000000;

    // console.log('*************', {
    //   from: god.toBase58(),
    //   to: creatorTokenAcc.toBase58(),
    //   toStorageAccount: storageTokenAcc.toBase58(),
    //   tokenProgram: TOKEN_PROGRAM_ID.toBase58(),
    //   programId: program.programId.toBase58(),
    // });

    await program.rpc.interaction(new anchor.BN(INTERACTION_FEE), {
      accounts: {
        from: god,
        to: creatorTokenAcc,
        toStorageAccount: storageTokenAcc,
        owner: program.provider.wallet.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      }
    });

    // assert.ok(checkAccount.from.equals(god));
    // assert.ok(checkAccount.to.equals(receiver));
    // assert.ok(checkAccount.amount.eq(new anchor.BN(100)));
    // assert.ok(checkAccount.memo === "Hello world");
    // assert.ok(checkAccount.vault.equals(vault.publicKey));
    // assert.ok(checkAccount.nonce === nonce);
    // assert.ok(checkAccount.burned === false);

    // let vaultAccount = await serumCmn.getTokenAccount(
    //   program.provider,
    //   checkAccount.vault
    // );
    // assert.ok(vaultAccount.amount.eq(new anchor.BN(100)));
  });
});
