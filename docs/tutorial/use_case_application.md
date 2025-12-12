# Use-case application

| Previous                                                | Next                                                | Up                                          |
|---------------------------------------------------------|-----------------------------------------------------|---------------------------------------------|
| [Partnerchain integration](partnerchain_integration.md) | [Operating instructions](operating_instructions.md) | [Node customization](node_customization.md) |

Here, we only give some broad hints on how we handled the integration of an app, since this is the
part that will depend the most on your particular use case.

From a [recent
survey](https://docs.google.com/document/d/1M6W_bv6s3-Q4HAr4zZCMtfjCjoutFDDXdZSeqiDeEAY/edit?tab=t.0#heading=h.fsivic5l6yjb),
we concluded that implementing a game would be the most convenient
example. Hence we chose **Asteria**, which showcases the capabilities of the
eUTxO model, particularly its concurrency benefits. To test our application, please copy the `game`
directory from our repo and integrate it as indicated below.

Those not familiar with the mechanics of the game can check the [game README](../../game/README.md#game-usage).

## Technical details

Asteria relies on three validators
to check every operation of the game.  It uses redeemers to specify game operations, and uses datums
to keep the game state. For those curious about the overall design of the application, we direct
you to its [design document](../../game/onchain/docs/design/design.md), that excellently
describes it. From now on we’ll discuss the technical aspects of implementing the game in our
Substrate node.

There are two sides to the implementation process, which we proceed to discuss right away.

### Offchain implementation

As the first step, we decided on an offchain transaction building approach. This is the
functionality that allows us to interact with the chain, and it is similar to that of regular
offchains we see on Cardano. This allowed us to integrate the game easily into the node with minimal
modifications to the node itself.

The complexity arises as the system does not have a lot of usual features like a balancer, e.g.,
every output has to be constructed individually taking the coin expenses in
consideration. Fortunately, there are no fees so that reduces the number of calculations to perform.

### Node integration

We first add `gpc-wallet` and the `game` to the workspace appropriately,

The key step is to integrate the game into the node via the definition of a command line interface
that allows the users to play the game and the modification of the genesis of the chain.

#### Genesis

An important decision made was to assume a fixed instance of the validators. This allows us to
initialize the chain with the game board in the genesis. In the file that holds the initial UTxO set
of the chain, we included the UTxOs for the prize at Asteria and some pellets. This means that as soon as the
chain starts, the user can enter the game by creating their ship.

The changes are as follows:

``` diff
diff --git a/runtime/src/genesis.rs b/runtime/src/genesis.rs
--- a/runtime/src/genesis.rs
+++ b/runtime/src/genesis.rs
@@ -12,6 +12,7 @@ pub const GENESIS_DEFAULT_JSON: &str = r#"
 {
     "zero_time": 1747081100000,
     "zero_slot": 0,
+    "slot_length": 3000,
     "outputs": [
         {
             "address": "6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4",
@@ -25,15 +26,45 @@ pub const GENESIS_DEFAULT_JSON: &str = r#"
             "datum": "820080"
         },
         {
-            "address": "61547932e40a24e2b7deb41f31af21ed57acd125f4ed8a72b626b3d7f6",
-            "coin": 314150000,
+            "address": "70bac1753d5f7e3609c92776371fd0eafa753889e5712858f48fb83981",
+            "coin": 500000000,
             "value": [
                     {
-                        "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
-                        "assets": [ ["tokenA", 300000000], ["tokenB", 2000000000] ]
+                        "policy": "516238dd0a79bac4bebe041c44bad8bf880d74720733d2fc0d255d28",
+                        "assets": [ ["asteriaAdmin", 1] ]
                     }
                     ],
-            "datum": "820080"
+            "datum": "D8799F00581C7BA97FB6E48018EF131DD08916939350C0CE7050534F8E51B5E0E3A4FF"
+        },
+        {
+            "address": "706a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
+            "coin": 2000,
+            "value": [
+                    {
+                        "policy": "6a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
+                        "assets": [ ["FUEL", 80] ]
+                    },
+                    {
+                        "policy": "516238dd0a79bac4bebe041c44bad8bf880d74720733d2fc0d255d28",
+                        "assets": [ ["asteriaAdmin", 1] ]
+                    }
+                    ],
+            "datum": "d8799f2703581c7ba97fb6e48018ef131dd08916939350c0ce7050534f8e51b5e0e3a4ff"
+        },
+        {
+            "address": "706a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
+            "coin": 2000,
+            "value": [
+                    {
+                        "policy": "6a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
+                        "assets": [ ["FUEL", 120] ]
+                    },
+                    {
+                        "policy": "516238dd0a79bac4bebe041c44bad8bf880d74720733d2fc0d255d28",
+                        "assets": [ ["asteriaAdmin", 1] ]
+                    }
+                    ],
+            "datum": "D8799F0521581C7BA97FB6E48018EF131DD08916939350C0CE7050534F8E51B5E0E3A4FF"
         },
         {
             "address": "0000000000000000000000000000000000000000000000000000000000",
```

#### Node commands

The main purpose of these functions is to simplify the user experience, as a friendlier layer on top
of the `tx-builder`. The game commands are innate to the node, as we want the game to be the main
functionality of the chain. These additions take place at `/node/src/cli.rs` and
`/node/src/command.rs`. These `GameCommands` have also been added to the game application, as
detailed in the [third dev-log](../dev_logs/use_case_implementation.md#add-game-crate) (from item 3
onwards).


| Previous | Next | Up |
|---------------------------------------------------------|-----------------------------------------------------|---------------------------------------------|
| [Partnerchain integration](partnerchain_integration.md) | [Operating instructions](operating_instructions.md) | [Node customization](node_customization.md) |

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
