# Game: Asteria

As part of the discovery process, it was decided that the use case for the partnerchain reference implementation would be a game. We decided on implementing Asteria which is a simple game about that showcases the capabilities of the eUTxO model. The game is simple: each player has its own ship that they can move according to a maximum speed to reach Asteria and claim a reward. 

In this document we’ll go over the [mechanics of the game](#brief-introduction-to-game-mechanics), to better understand the requirements, then we’ll lay out the [design decisions and technical aspects](#technical-details) of the implementation, and finally, we'll detail the [usage of the game](#game-usage) via node commands.

## Brief introduction to game mechanics

The game happens within a 2D grid through which `ships` can travel by moving a certain `distance` that does not surpass the `maximum speed`. The `Asteria` is at the center of the grid, at the `(0,0) coordinates`. Dispersed through the grid at fixed coordinates are `pellets`, which are freely available sources of fuel that ships can consume when they are sitting on those same coordinates.
The distance between the Asteria and the ships, or the ship’s before and after moving, is measured using [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry).

The game is ruled over by on chain validators that are parameterised. Because of this, we can have various games with different settings. The most relevant parameters for our case are the following:

- `Min_asteria_distance`: minimum distance between the Asteria and a new ship on its creation.
- `Max_ship_fuel`: maximum amount of fuel that a ship can have.
- `Max_asteria_mining`: maximum percentage of the reward pool that a single ship can withdraw upon arriving at Asteria
- `Max_speed`: maximum distance a ship can travel in a certain time.
- `Fuel_per_step`: how much fuel is consumed when moving the ship 1 step.

And we have the following configuration:

- `Min_asteria_distance`: 10
- `Max_ship_fuel`: 30
- `Max_asteria_mining`: 50
- `Max_speed`: 1 step per 30 seconds.
- `Fuel_per_step`: 1 fuel per step.

### Game Operations

Let’s see the details of the operations that our node supports and their constraints:
- *Create ship*: Places a new ship on the map, according to the coordinates provided by the user. This initial placement needs to abide by the previously mentioned `min_asteria_distance`. 
- *Move ship*: Moves a ship to the coordinates provided by the user if:
the movement complies with the max_speed, and
the ship has enough fuel to travel
- *Gather fuel*: Collects fuel from a pellet if the ship’s coordinates overlap the pellet’s.
- *Mine Asteria*: Collects a portion of the reward if the ship’s coordinates overlap Asteria’s and the portion to be redeemed satisfies `max_asteria_mining`.

There are more operations that we don’t support, and there are other validations done upon these operations we mentioned, but for the sake of simplicity we don’t go over every single one of the validations. For a more thorough explanation, you can check [Asteria’s official repository](https://github.com/txpipe/asteria).

## Technical details

Asteria is implemented utilizing the capabilities of the eUTxO model. It relies on three validators to check every operation of the game.  It uses redeemers to specify game operations, and uses datums to keep the game’s state. We won’t discuss the overall design of the application in the eUTxO model as there is already a [design document](./design/design.md) that excellently describes it. From now on we’ll describe the technical aspects of implementing the game in our Substrate node.

### Offchain implementation
As the first step, we decided on an offchain transaction building approach. This is the functionality that allows us to interact with the chain, and it is similar to that of regular offchains we see on Cardano. This allowed us to integrate the game easily into the node with minimal modifications to the node itself.
The complexity arises as the system does not have a lot of usual features like a balancer, e.g., every output has to be constructed individually taking the coin expenses in consideration. Fortunately, there are no fees so that reduces the calculations to perform. 
 
### Node integration

The key step is to integrate the game into the node via the definition of a command line interface that allows the users to play the game and the modification of the genesis of the chain.

#### Genesis

An important decision made was to assume a fixed instance of the validators. This allows us to initialize the chain with the game board in the genesis. In the file that holds the initial UTxO set of the chain, we included the UTxOs for the Asteria and some pellets. This means that as soon as the chain starts, the user can enter the game by creating their ship.

#### Node commands

The main purpose of these functions is to simplify the usage for the user, as they could interact with the game using the `tx-builder` as well. The game commands are innate to the node, as we want the game to be the main functionality of the chain.

## Game usage

Each of the following actions must be run with a running instance of the node.

### Deploy Scripts

This command reads all the script parameters provided in the argument JSON file and applies them to the generic (parameterized)
scripts, writing the resulting ones in their respective files, inside the `scripts_directory` specified in the same JSON file.

```console
./target/release/griffin-partner-chains-node game deploy-scripts
--params-path <GAME_PARAMS_PATH>
```

#### Arguments details:

*params-path*: path to the JSON file containing all the game scripts parameters and the target directory to write the resulting scripts.

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

This command moves the ship to a different point in the grid (updates de `pos_x` and `pos_y` fields in the ship datum). The transaction also burns the fuel tokens consumed.

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
- *ttl*: the transaction’s time-to-live. The resulting POSIX time of the validity interval is used to set the initial `last-move-latest-time` field in the Ship output datum. The manhattan distance travelled divided by the POSIX validity range must be less or equal to the max speed.

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


