## Useful commands

- anchor deploy --provider.cluster devnet // !!!! this will reset the Program Id
- anchor build - then use solana deploy (use --verifiable flag to build a consistent with the docker image)
- solana program deploy 'sosol-program/target/verifiable/boom.so'
- anchor test --skip-local-validator
- spl-token create-token /path/to/keypair.json
- spl-token create-account boomh1LQnwDnHtKxWTFgxcbdRjPypRSjdwxkAEJkFSH
- spl-token mint boomh1LQnwDnHtKxWTFgxcbdRjPypRSjdwxkAEJkFSH 420000000

## Program

The program ID is `BooManQtsP9pBNudF2HDGNT9xkjL63BiWVWpfkvLkmQW` and the key has been backed up to ensure that it will remain so in all future deployments.

- key stored in `sosol-program/target/verifiable/boom-keypair.json`
- Can be overidden with `solana program deploy --program-id <KEYPAIR_FILEPATH> <PROGRAM_FILEPATH>`

### Verify program

Verify the program is correct using `anchor verify --provider.cluster devnet -p boom BooManQtsP9pBNudF2HDGNT9xkjL63BiWVWpfkvLkmQW`
