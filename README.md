# Fun stuff for memorycollector

```bash
cd contract
./build.sh
./test.sh
```

## Try it out on testnet

Replace `vec.mike.testnet` and `mike.testnet` accordingly below:

```bash
near create-account vec.mike.testnet --masterAccount mike.testnet

near deploy vec.mike.testnet --wasmFile res/vec_issue_memorycollector.wasm --initFunction new --initArgs '{}'

near call vec.mike.testnet insert --accountId mike.testnet

near view vec.mike.testnet get_index '{"idx": "4"}'

near view vec.mike.testnet get_index '{"idx": "0"}'
```

## Troubleshooting

Delete and recreate the account with:

    near delete vec.mike.testnet mike.testnet && near create-account vec.mike.testnet --masterAccount mike.testnet