# Development Tutorial

In this Tutorial, we will walk you through all the steps necessary to customize a generic Substrate
node into a Partnerchain node with use-case specific features.
 
The material presented here is a streamlined version of the info detailed at the [Dev Activity
Logs](../dev_logs/) ([node customization](../dev_logs/initial_customizations.md), [Partner Chains SDK
integration](../dev-logs/partner_chain_integration.md)).

## Summary

This reference implementation takes the very basic [Substrate's minimal
template](https://github.com/paritytech/polkadot-sdk-minimal-template) and shows how to: 

- add a custom ledger (eUTxO using Griffin);
- set up consensus (Aura) and finality (GRANDPA) algorithms;
- integrate the Partner Chains (PC) SDK; and
- set up an application (Asteria).

We hope that the guide presented here helps you to set your particular use-case.

## Index

- [Node customization](node_customization.md) involves:

  - [Installing the ledger](node_customization.md#installing-the-ledger) explains the process of adding the Griffin
    ledger. This requires extensive editing of the runtime, which is detailed in [Runtime
    sources](node_customization.md#modifications-at-the-runtime-sources). The node client requires fewer modifications, which are detailed in
    [Node Sources](node_customization.md#modifications-at-the-node-sources).

  - [Partnerchain integration](partnerchain_integration.md) explains how to integrate the PC SDK
    into our node; in particular, that it can be used without relying on FRAME pallets.

  - [Use-case application](use_case_application.md) gives broad hints on how to integrate your
    application to a modified node.

- [Operating instructions](operating_instructions.md) indicates how to setup the node to work with a
  as a partner chain, details how to [run an example on
  devnet](operating_instructions.md#example-with-devnet), ....

- [Troubleshooting](troubleshooting.md) addresses some common pitfalls while editing and building
  Substrate nodes.

| Next                                        | Up                           |
|---------------------------------------------|------------------------------|
| [Node customization](node_customization.md) | [Root README](../../README.md) |

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
