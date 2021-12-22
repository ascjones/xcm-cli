# xcm-cli

## Teleport Asset

E.g. teleporting to Canvas parachain on Rococo:

```sh
xcm-cli teleport \
      --url wss://rococo-rpc.polkadot.io:443 \
      --parachain-id 1002 \
      --dest-account <dest public account> \
      --amount 10000000000000 \
      --suri <your private key hex>
```