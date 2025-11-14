# Dev activity log: Use case specific features

This is the deatiled log of the development activities required for the implementation of the use case specific features. The use case to develop was a game and we decided to implement Asteria. This is a game with 4 main operations that utilize the eUtxO model, so the task was to implement each of them. A more in-depth explanation of the game's operations and design can be found in the [onchain documentation](../../game/onchain/docs/design/design.md).
This log is divided in three parts: 
- [_game integration_](#add-game-crate): step-by-step modifications that were made to implement the game features,
- [_game commands in the node_](#add-game-to-node-commands): add the operations as commands to the node,
- [_wallet refactor_]: particular to our node, we refactored the wallet to make it more modular,
- [_game usage_]: detailed instructions on how to use the commands and play the game. 

## Add Game Crate

1. Add the aiken (on-chain) code and its documentation in `game/onchain`.

2. Define the game crate and add it to the `Cargo.toml` as a member of the workspace:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/b96196d87dadbcf194fe6700aa3236ead7b1dd34/game/Cargo.toml#L1-L23

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/b96196d87dadbcf194fe6700aa3236ead7b1dd34/Cargo.toml#L11-L19

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/b96196d87dadbcf194fe6700aa3236ead7b1dd34/Cargo.toml#L82


3. Define new `GameCommand` struct and implement its `run` method:

We define an overarching `GameCommand` enum that will hold the actual subcommands enum, this structure allows us to integrate the command into the node.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L9-L13


The `run` method dictates the parsing of the commands:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L29-L68

4. Define each game command:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L15-L27

5. Define each game commands' arguments:

    1. Arguments for CreateShip:

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L70-L107
    
    2. Arguments for MoveShip:

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L109-L139

    3. Arguments for GatherFuel:

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L141-L188

    4. Arguments for MineAsteria:

    https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/lib.rs#L190-L217

4. Define the logic of each command (the functions called in `lib.rs`) in `game.rs`. This constitutes our off-chain code. These modifications are too long to add them all but you can check them out fully [here](../../game/src/game.rs). 

We will higlight some key deatils of these implementations:

- Example of a redeemer:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/game.rs#L130-L141

- Example of a datum:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/game.rs#L167-L174

- Example of an output:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/game.rs#L192-L198

- Example of asset burning:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/game.rs#L764-L767

5. Add `deploy_scripts` function. This command allows the user to generate new instances of the parameterized scripts to create new games. 
This command searches for a file that has a structure like this:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/deploy_params.json#L1-L14

This function is also quite long to icnlude it as a whole, but there are some details worth highlighting:

- Example of building the Plutus Data for the parameter application:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/game.rs#L944-L959

- Example of applying parameters to a script:

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/game/src/game.rs#L961-L967

## Add game to node commands

1. Include the `game` crate as a dependency in the node's `Cargo.toml`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/node/Cargo.toml#L25

2. Add a new `Game(GameCommand)` item in the `Subcommand` enum, within `node/src/cli.rs`.

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/node/src/cli.rs#L112-L114

3. Include the previous item in the subcommands match struct, within the `run` function in `node/src/command.rs`. 

https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/1b98b2b0fd645ec32e071a1ecb9b6a5829176057/node/src/command.rs#L59-L63

