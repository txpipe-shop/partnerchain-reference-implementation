Griffin Wallet
==============

This CLI wallet is based on a minimized version of the [Tuxedo wallet](https://github.com/Off-Narrative-Labs/Tuxedo/tree/main/wallet). It is provided for demonstration purposes of the UTxO features of the Griffin Solochain Node.

## Installation

You should have a properly installed Griffin node to build the wallet. After following the [instructions to do that](https://github.com/txpipe/griffin/blob/main/README.md#installation), run

```bash
cargo build --release -p griffin-wallet
```

As explained in the node installation instructions, omitting the `--release` will build the "debug" version.

## Basic usage

In terminal, run the node in development mode:

```bash
./target/release/griffin-solochain-node --dev
```

In another terminal, one can interact with the node by issuing wallet commands. Every time the wallet starts (without the `--help` or `--version` command-line options), it will try to synchronize its database with the present chain state, unless there is a mismatch with the genesis hash.

To list the whole UTxO set, run

```bash
./target/release/griffin-wallet show-all-outputs
```

When this is done for the first, the output will look like this:

```
[2024-11-14T12:37:20Z INFO  griffin_wallet] Number of blocks in the db: 5
[2024-11-14T12:37:20Z INFO  griffin_wallet] Wallet database synchronized with node to height 6
###### Unspent outputs ###########
998f074b5357d465fdd99198c65af6a418522e5a1688e2674c935702fef38d0600000000: owner address 6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4, datum Some(CuteOutput), amount: 314000000 Coins, Multiassets:
  (0x0298…2005) tokenA: 271000000
  (0x0298…2005) tokenB: 1123581321
```
This “genesis” UTxO belongs to Shawn's address. In order to spend it, we need to add his public/secret key pair (pk/sk) to the wallet keystore. We do this by generating the pair with the corresponding seed phrase:

```
$ ./target/release/griffin-wallet insert-key "news slush supreme milk chapter athlete soap sausage put clutch what kitten"

[2024-11-14T12:38:19Z INFO  griffin_wallet] Number of blocks in the db: 6
[2024-11-14T12:38:19Z INFO  griffin_wallet] Wallet database synchronized with node to height 26
The generated public key is 7b155093789404780735f4501c576e9f6e2b0a486cdec70e03e1ef8b9ef99274 (5Er65XH4...)
Associated address is 0x6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4
```

We use the `generate-key` command to have another pk/sk and address available for experimenting.

```
$ ./target/release/griffin-wallet generate-key

[2024-11-14T12:38:53Z INFO  griffin_wallet] Number of blocks in the db: 26
[2024-11-14T12:38:53Z INFO  griffin_wallet] Wallet database synchronized with node to height 37
Generated public key is 3538f889235842527b946255962241591cdc86cb99ba566afde335ae94262ee4 (5DGVKT7k...)
Generated Phrase is "vibrant assume service vibrant six unusual trumpet ten truck raise verify soft"
Associated address is 0x614fdf13c0aabb2c2e6df7a0ac0f5cb5aaabca448af8287e54681273dd
```

Now we spend the output, generating a new UTxO for the last address:

```
$ ./target/release/griffin-wallet spend-value --input 998f074b5357d465fdd99198c65af6a418522e5a1688e2674c935702fef38d0600000000 --amount 200000000 --recipient 0x614fdf13c0aabb2c2e6df7a0ac0f5cb5aaabca448af8287e54681273dd

[2024-11-14T12:41:18Z INFO  griffin_wallet] Number of blocks in the db: 37
[2024-11-14T12:41:18Z INFO  griffin_wallet] Wallet database synchronized with node to height 86
Note: Excess input amount goes to Shawn.
[2024-11-14T12:41:18Z INFO  griffin_wallet::money] Node's response to spend transaction: Ok("0x5a1974d3e3d32c075b220513125c9457ac9efc59a651d36704c0c7a4e389b6e6")
Transaction queued. When accepted, the following UTxOs will become available:
"dcb998d9e000c19fd20e41afeff6e1e0d9366e6e6c756c8173e52fc8061638f600000000" worth Coin(200000000).
"dcb998d9e000c19fd20e41afeff6e1e0d9366e6e6c756c8173e52fc8061638f601000000" worth Multiasset(114000000, EncapBTree({0x0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005: EncapBTree({AssetName("tokenA"): 271000000, AssetName("tokenB"): 1123581321})})).
```
>As you can see in the example above, the hash of the submitted extrinsic returned by the node differs from the transaction hash computed by the wallet. We rely on the latter to uniquely identify UTxOs. The first is [not guaranteed to be unique](https://docs.polkadot.com/polkadot-protocol/parachain-basics/blocks-transactions-fees/transactions/#unique-identifiers-for-extrinsics).

All command-line arguments admit short versions (run `./target/release/griffin-wallet -h` for details). The next invocation spends the first UTxO and sends some coins back to Shawn:

```
$ ./target/release/griffin-wallet spend-value --input dcb998d9e000c19fd20e41afeff6e1e0d9366e6e6c756c8173e52fc8061638f600000000 --amount 150000000 --witness 3538f889235842527b946255962241591cdc86cb99ba566afde335ae94262ee4

[2024-11-14T12:47:45Z INFO  griffin_wallet] Number of blocks in the db: 184
[2024-11-14T12:47:45Z INFO  griffin_wallet] Wallet database synchronized with node to height 215
Note: Excess input amount goes to Shawn.
[2024-11-14T12:47:45Z INFO  griffin_wallet::money] Node's response to spend transaction: Ok("0xbcc0e3f157c660e022890ea9a8ddf1e7a324dd7ae30496a774d4f04046b5097a")
Transaction queued. When accepted, the following UTxOs will become available:
"bf73bc5bcf3afa75a7070041c635d78f6613aa3b753956e93053077cf9dc4b8e00000000" worth Coin(150000000).
"bf73bc5bcf3afa75a7070041c635d78f6613aa3b753956e93053077cf9dc4b8e01000000" worth Coin(50000000).
```

In this second example, we had to explicitly state the pk of the owning address to allow spenditure; in order to be successful, the sk must be stored in the wallet's keystore. (If the `--witness` argument is missing, Shawns pk is implied, cf. the first spend.)

The UTxO set at this point is

```
$ ./target/release/griffin-wallet show-all-outputs

[2024-11-14T12:48:44Z INFO  griffin_wallet] Number of blocks in the db: 215
[2024-11-14T12:48:44Z INFO  griffin_wallet] Wallet database synchronized with node to height 234
###### Unspent outputs ###########
bf73bc5bcf3afa75a7070041c635d78f6613aa3b753956e93053077cf9dc4b8e00000000: owner address 6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4, datum None, amount: 150000000 Coins
bf73bc5bcf3afa75a7070041c635d78f6613aa3b753956e93053077cf9dc4b8e01000000: owner address 6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4, datum None, amount: 50000000 Coins
dcb998d9e000c19fd20e41afeff6e1e0d9366e6e6c756c8173e52fc8061638f601000000: owner address 6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4, datum None, amount: 114000000 Coins, Multiassets:
  (0x0298…2005) tokenA: 271000000
  (0x0298…2005) tokenB: 1123581321

```

Finally, to send some coins *and* `tokenA`s from the last UTxO to the other account, we do:
```
$ ./target/release/griffin-wallet spend-value --input dcb998d9e000c19fd20e41afeff6e1e0d9366e6e6c756c8173e52fc8061638f601000000 --amount 14000000 --policy 0x0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005 --name tokenA --token-amount 200000000 --recipient 0x614fdf13c0aabb2c2e6df7a0ac0f5cb5aaabca448af8287e54681273dd

[2024-11-14T12:54:28Z INFO  griffin_wallet] Number of blocks in the db: 250
[2024-11-14T12:54:28Z INFO  griffin_wallet] Wallet database synchronized with node to height 349
Note: Excess input amount goes to Shawn.
[2024-11-14T12:54:28Z INFO  griffin_wallet::money] Node's response to spend transaction: Ok("0xa7ad4765e2ab4767e434fc6c117929a8871288c094a428164071c63bd9f0490a")
Transaction queued. When accepted, the following UTxOs will become available:
"ae2bcf3d0b2ace1f957176f17bac72e3fc2e518c82b41a9bdd622bb82318e4b200000000" worth Multiasset(14000000, EncapBTree({0x0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005: EncapBTree({AssetName("tokenA"): 200000000})})).
"ae2bcf3d0b2ace1f957176f17bac72e3fc2e518c82b41a9bdd622bb82318e4b201000000" worth Multiasset(100000000, EncapBTree({0x0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005: EncapBTree({AssetName("tokenA"): 71000000, AssetName("tokenB"): 1123581321})})).
```

The *balance* summarizes `Value` amounts for each address:

```
$ ./target/release/griffin-wallet show-balance

[2024-11-14T12:54:34Z INFO  griffin_wallet] Number of blocks in the db: 349
[2024-11-14T12:54:34Z INFO  griffin_wallet] Wallet database synchronized with node to height 351
Balance Summary
6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4: 300000000 Coins, Multiassets:
  (0x0298…2005) tokenA: 71000000
  (0x0298…2005) tokenB: 1123581321
614fdf13c0aabb2c2e6df7a0ac0f5cb5aaabca448af8287e54681273dd: 14000000 Coins, Multiassets:
  (0x0298…2005) tokenA: 200000000
--------------------
total      : 314000000 Coins, Multiassets:
  (0x0298…2005) tokenA: 271000000
  (0x0298…2005) tokenB: 1123581321
```


## Complete Transaction Builder

In order to reproduce more complex wallet commands, like consuming a script input or minting an asset, we provide a more complete transaction builder via the `build-tx` command. The only argument is a JSON file with all the necessary information about inputs, outputs, scripts, mintings, witnesses, required signers and validity interval. Run the command with:

```bash
$ ./target/release/griffin-wallet build-tx --tx-info /path/to/your/json/file.json
```

The json file must contain the following fields:

- `inputs_info`: A list of input information objects.
   Each input info contains the following fields:
    - `tx_hash`: The hash of the transaction containing the output to be used as input.
    - `index`: The index of the output in the transaction.
    - `redeemer_cbor`: The cbor-encoded redeemer (optional, for script inputs).
- `outputs_info`: A list of output information objects.
   Each output info contains the following fields:
    - `address`: The address of the output.
    - `coin`: An amount of `Coin`s to be included in the output.
    - `value`: A list of assets to be included in the output.
      Each asset entry contains the following fields:
        - `policy`: The policy ID of the asset bundle.
        - `assets`: A list of tuples containing the asset name and the amount to be included.
    - `datum`: The hex-encoded datum (optional, for script outputs).
- `scripts_info`: A list of JSON objects containing the hex of plutus scripts
   and their parameters (if any) to be applied to the scripts.
   Each object must contain the following fields:
    - `script_hex`: The hex-encoded script.
    - `script_params_cbor`: The cbor-encoded parameter list (optional).
- `mintings_info`: A list of minting information objects (optional).
   Each minting info contains the following fields:
    - `policy`: The policy ID of the asset to be minted/burnt.
    - `assets`: A list of tuples containing the asset name and the amount to be minted/burnt.
    - `redeemer_cbor`: The cbor-encoded redeemer to the minting policy.
- `witnesses`: List of public keys.
- `required_signers`: List of payment hashes.
- `validity_interval_start`: Start of the validity interval (optional).
- `ttl`: Time to live (optional).

Keep in mind that with this command the correct balance of the transaction must be ensured by the user.

### Example JSON files

There are example contracts and json files for testing this command in the `eutxo_examples` directory.
These examples use the following credentials that match addresses actually present in the genesis UTxO set, so they can be run right away:

```rust
pub const SHAWN_PHRASE: &str =
    "news slush supreme milk chapter athlete soap sausage put clutch what kitten";

/// The public key corresponding to the seed above.
pub const SHAWN_PUB_KEY: &str = "7b155093789404780735f4501c576e9f6e2b0a486cdec70e03e1ef8b9ef99274";

/// The public key hash corresponding to Shawn's public key.
pub const SHAWN_PUB_KEY_HASH: &str = "01e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4";

/// The address corresponding to Shawn's public key. Such addresses always start with `0x61`.
pub const SHAWN_ADDRESS: &str = "6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4";

/// Extra credentials for spending genesis outputs with matching address.
pub const ALICE_PHRASE: &str =
    "mobile broken meat lonely empty cupboard mom come derive write sugar cushion";

pub const ALICE_PUB_KEY: &str = "c6f58a018f5f4fba0eec76bb2d92cacab00eb6b548197925572c61c17b3e4edf";

pub const ALICE_PUB_KEY_HASH: &str = "547932e40a24e2b7deb41f31af21ed57acd125f4ed8a72b626b3d7f6";

pub const ALICE_ADDRESS: &str = "61547932e40a24e2b7deb41f31af21ed57acd125f4ed8a72b626b3d7f6";
```

## Queries

Apart from getting the whole UTxO set or the balance, one can also filter UTxOs by address or by asset. For example, to get all UTxOs owned by Shawn's address:

```
$ ./target/debug/griffin-wallet show-outputs-at --address 6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4
```

or to get all UTxOs containing `tokenA` with policy ID `0x0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005`:

```
$ ./target/debug/griffin-wallet show-outputs-with-asset --policy 0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005 --name tokenA
```

Both commands will print the corresponding UTxOs in the same format as `show-all-outputs`.

## Help
For a complete list of commands and options, run

```bash
./target/release/griffin-wallet --help
```
