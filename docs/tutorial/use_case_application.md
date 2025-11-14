# Use-case application

| Previous                                                | Next                                                | Up                                          |
|---------------------------------------------------------|-----------------------------------------------------|---------------------------------------------|
| [Partnerchain integration](partnerchain_integration.md) | [Operating instructions](operating_instructions.md) | [Node customization](node_customization.md) |

From a [recent
survey](https://docs.google.com/document/d/1M6W_bv6s3-Q4HAr4zZCMtfjCjoutFDDXdZSeqiDeEAY/edit?tab=t.0#heading=h.fsivic5l6yjb),
we concluded that implementing a game would be the most convenient
example. Hence we chose **Asteria**, which showcases the capabilities of the
eUTxO model, particularly its concurrency benefits.

Those not familiar with  mechanics of the game can check the [game README](../../game/README.md#game-usage).

## Technical details

Asteria relies on three validators
to check every operation of the game.  It uses redeemers to specify game operations, and uses datums
to keep the game state. For those curious about the overall design of the application, we direct
you to its [design document](../../game/onchain/docs/design/design.md), that excellently
describes it. From now on we’ll discuss the technical aspects of implementing the game in our
Substrate node.

There are two sides to the implementation process, which we proceed to discuss rightaway

### Offchain implementation

As the first step, we decided on an offchain transaction building approach. This is the
functionality that allows us to interact with the chain, and it is similar to that of regular
offchains we see on Cardano. This allowed us to integrate the game easily into the node with minimal
modifications to the node itself.

The complexity arises as the system does not have a lot of usual features like a balancer, e.g.,
every output has to be constructed individually taking the coin expenses in
consideration. Fortunately, there are no fees so that reduces the number of calculations to perform.

### Node integration

The key step is to integrate the game into the node via the definition of a command line interface
that allows the users to play the game and the modification of the genesis of the chain.

#### Genesis

An important decision made was to assume a fixed instance of the validators. This allows us to
initialize the chain with the game board in the genesis. In the file that holds the initial UTxO set
of the chain, we included the UTxOs for the prize at Asteria and some pellets. This means that as soon as the
chain starts, the user can enter the game by creating their ship.

#### Node commands

The main purpose of these functions is to simplify the user experience, as a friendlier layer on top
of the `tx-builder`. The game commands are innate to the node, as we want the game to be the main
functionality of the chain.

| Previous | Next | Up |
|---------------------------------------------------------|-----------------------------------------------------|---------------------------------------------|
| [Partnerchain integration](partnerchain_integration.md) | [Operating instructions](operating_instructions.md) | [Node customization](node_customization.md) |

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
