# Game usage

| Previous                                            | Next                                  | Up                         |
|-----------------------------------------------------|---------------------------------------|----------------------------|
| [Operating instructions](operating_instructions.md) | [Troubleshooting](troubleshooting.md) | [Tutorial root](README.md) |


In this final section we explain how to use our particular application.

> [!NOTE]
>
> Each of the following actions must be run with a running instance of the node.

## Deploy Scripts

This command reads all the script parameters from `game/src/deploy_params.json` and applies them to the generic (parameterized)
scripts, writing the resulting ones in their respective files, inside the `scripts` directory.

```console
./target/release/griffin-partner-chains-node game deploy-scripts
--params <SCRIPTS_PARAMS_PATH>
```

### Arguments details:

*params*: path to the JSON file containing all the script parameters.

### Create Ship

This command creates the player’s Ship. The transaction also mints the initial ship’s fuel, the ship and pilot tokens, and pays an inscription fee that is added to the total prize in the Asteria UTxO. The pilot token goes back to the wallet input owner, and serves as a proof of the ownership of the Ship.

```console
./target/release/griffin-partner-chains-node game create-ship
--input <WALLET_OUTPUT_REF>
--witness <PUBLIC_KEY>
--pos-x <POS_X>
--pos-y <POS_Y>
--ttl <TIME_TO_LIVE>
```
#### Arguments details:

- *input*: a wallet input that must be consumed to pay for the minimal amount of coin in the Ship output and the fee added to the Asteria accumulated rewards.
- *witness*: public key of the input owner. If omitted, Shawn’s pub key is the default value, since this makes it easier to test transactions in a `dev` environment.
- *pos-x*: initial “x” coordinate of the Ship output.
- *pos-y*: initial “y” coordinate of the Ship output.
- *ttl*: the transaction’s time-to-live. The resulting POSIX time of the validity interval is used to set the initial `last-move-latest-time` field in the Ship output datum.

### Gather Fuel

This command moves fuel tokens from a pellet UTxO to a ship UTxO, only if they have the same position in the grid, as specified in the datums. The amount of fuel to gather is specified in the redeemer, and the total ship fuel must not exceed its maximum capacity.

```console
./target/release/griffin-partner-chains-node game gather-fuel
--ship <SHIP_OUTPUT_REF>
--pellet <PELLET_OUTPUT_REF>
--witness <PUBLIC_KEY>
--fuel <FUEL_AMOUNT>
--validity-interval-start <VALIDITY_INTERVAL_START>
```

#### Arguments details:

- *ship*: reference to the ship UTxO.
- *pellet*: reference to the pellet UTxO.
- *witness*: public key of the pilot token owner. This is necessary since the pilot UTxO must be provided as input to prove the ship ownership. If omitted, Shawn’s pub key is the default value.
- *fuel*: the amount of fuel to transfer from the pellet to the ship.
- *validity-interval-start*: start of the transaction’s validity interval. The corresponding POSIX must be greater than the `last-move-latest-time` field in the ship datum, in order to respect the speed limit of the last move.

### Move Ship

This command moves the ship to a different point in the grid (updates the `pos_x` and `pos_y` fields in the ship datum). The transaction also burns the fuel tokens consumed.

```console
./target/release/griffin-partner-chains-node game move-ship
--ship <SHIP_OUTPUT_REF>
--witness <PUBLIC_KEY>
--pos-x <POS_X>
--pos-y <POS_Y>
--validity-interval-start <VALIDITY_INTERVAL_START>
--ttl <TIME_TO_LIVE>
```

#### Arguments details:

- *ship*: reference to the ship UTxO.
witness: public key of the pilot token owner. This is necessary since the pilot UTxO must be provided as input to prove the ship ownership. If omitted, Shawn’s pub key is the default value.
- *pos-x*: new “x” coordinate of the ship.
- *pos-y*: new “y” coordinate of the ship.
- *validity-interval-start*: start of the transaction’s validity interval. The corresponding POSIX must be greater than the `last-move-latest-time` field in the ship datum, in order to respect the speed limit of the last move.
- *ttl*: the transaction’s time-to-live. The resulting POSIX time of the validity interval is used to set the initial `last-move-latest-time` field in the Ship output datum. The Manhattan distance traveled divided by the POSIX validity range must be less or equal to the max speed.

### Mine Asteria

This command can be triggered when the ship reaches Asteria, i.e., its coordinates are both zero. Then the ship owner can receive a percentage of the total prize given by (MAX_ASTERIA_MINING/100). This transaction also burns the ship and all remaining fuel tokens.

```console
./target/release/griffin-partner-chains-node game mine-asteria
--ship <SHIP_OUTPUT_REF>
--witness <PUBLIC_KEY>
--validity-interval-start <VALIDITY_INTERVAL_START>
```

#### Arguments details:

- *ship*: reference to the ship UTxO.
- *witness*: public key of the pilot token owner. This is necessary since the pilot UTxO must be provided as input to prove the ship ownership. If omitted, Shawn’s pub key is the default value.
- *validity-interval-start*: start of the transaction’s validity interval. The corresponding POSIX must be greater than the `last-move-latest-time` field in the ship datum, in order to respect the speed limit of the last move.

| Previous                                            | Next                                  | Up                         |
|-----------------------------------------------------|---------------------------------------|----------------------------|
| [Operating instructions](operating_instructions.md) | [Troubleshooting](troubleshooting.md) | [Tutorial root](README.md) |

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
