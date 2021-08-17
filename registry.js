const anchor = require("@project-serum/anchor");
const provider = anchor.Provider.env();
anchor.setProvider(provider);

describe("sosol-tests", () => {
  const registry_idl = JSON.parse(
    require("fs").readFileSync("./tests/registry.json", "utf8")
  );
  const programId = new anchor.web3.PublicKey(
    "E4bVQvJ1e961cks6jAM7gXy3u4ziy9mrGWzBCuJ8zj8G"
  );
  const registry = new anchor.Program(registry_idl, programId);
  it("should intereact with the program", async () => {
    const allRewards = await registry.account.rewardVendor.all();

    const parsedRewards = allRewards.map((x) => {
      return {
        expired: x.account.expired,
        expiryReceiver: x.account.expiryReceiver.toBase58(),
        expiryTs: parseInt(x.account.expiryTs),
        from: x.account.from.toBase58(),
        kind: Object.keys(x.account.kind)[0],
        mint: x.account.mint.toBase58(),
        nonce: x.account.nonce,
        poolTokenSuppy: parseInt(x.account.poolTokenSupply),
        registar: x.account.registrar.toBase58(),
        rewardEventQCursor: x.account.rewardEventQCursor,
        startTS: parseInt(x.account.startTs),
        total: parseInt(x.account.total),
        vault: x.account.vault.toBase58(),
      };
    });
    console.log(parsedRewards);
  });
});
