# Asteria Game Example

This is an example of a simple on-chain game built using the Extended UTXO model. In this game, players can use their ship UTxOs to gather fuel, and use it to move their spaceships to mine resources from the Asteria UTxO, at the center of the game board. You can find more details about the game mechanics in the [game document](../../../../game/README.md).

## Running the Example

You must have a running instance of the Partnerchain node, and run the trace in order using the wallet tx-builder, like in the following example:

```bash
./target/release/griffin-wallet wallet build-tx --tx-info wallet/src/eutxo_examples/game/trace_example/1_create_ship.json 
```

Keep in mind that:
- Each transaction `validity_interval_start` and `ttl` are measured in slots. With the default genesis configuration, slots are 3 seconds long.
- The create-ship transaction has a `ttl` of 20, so you must submit it before the corresponding slot is reached (with the default genesis configuration, this means that you must do it within 60 seconds after starting the node).
- In order to respect the time of ship creation, the gather-fuel transaction has a `validity_interval_start` of 20. This means you have to _wait_ until slot 20 is reached before submitting it.
- Similarly, the move-ship transaction has a `validity_interval_start` of 20, so you can run it right after the gather-fuel transaction is confirmed. This transaction has a high `ttl` of 130 in order to respect the max speed of the ship.
- Finally, the mine-asteria transaction has a `validity_interval_start` equal to the previous transaction `ttl`, i.e., 130, so you have to wait until that slot is reached before submitting it.
