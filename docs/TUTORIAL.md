# Development Tutorial

In this Tutorial, we will walk you through all the steps necessary to customize a generic Substrate
node into a Partnerchain node with use-case specific features.
 
The material presented here is a streamlined version of the info detailed at the [Dev Activity
Logs](dev_logs/) ([node customization](dev_logs/initial_customizations.md), [Partner Chains SDK
integration](dev-logs/partner_chain_integration.md)).

## Summary

This reference implementation takes the very basic [Substrate's minimal
template](https://github.com/paritytech/polkadot-sdk-minimal-template) and shows how to: 

- add a custom ledger (eUTXO using Griffin);
- set up consensus (Aura) and finality (GRANDPA) algorithms;
- integrate the Partner Chains (PC) SDK; and
- set up an application (Asteria).

We hope that the guide presented here helps you to set your particular use-case.

## Node Customization

### Requisites

For concreteness' sake, we run this tutorial on Linux. The following required elements are specified
with the corresponding tested versions.

1. Sources from the [PSDK minimal
   template](https://github.com/paritytech/polkadot-sdk-minimal-template) repo, on its
   [update](https://github.com/paritytech/polkadot-sdk-minimal-template/commit/c6f71c73f61bc8bc78574037fed712f8046b57c7)
   to PSDK `stable2503` (the latest one at the time of this writing).
2. Rust `1.90.0-x86_64-unknown-linux-gnu` and the additional dependencies needed for your system
   (check Polkadot's official [installation
   instructions](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/) for that).
3. Rust's target `wasm32v1-none` and the
   `rust-src` component, which can be installed by executing the following commands:

   ```bash
   $ rustup target add wasm32v1-none --toolchain stable-x86_64-unknown-linux-gnu
   $ rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
   ```
4. Sources from IOG's [Partner Chains (PC)
   SDK](https://github.com/input-output-hk/partner-chains/tree/master) at [release
   `v1.8.0`](https://github.com/input-output-hk/partner-chains/releases/tag/v1.8.0).
5. The modified [Griffin](https://github.com/txpipe/griffin/releases/tag/catalyst-ms5/) sources at
   this repo (comprising the directories [griffin-core](../griffin-core),
   [griffin-rpc](../griffin-rpc), [wallet](../wallet), and [demo](../demo)).

For a quick summary of the Substrate Node components, check [this Section of the
dev-logs](https://github.com/txpipe-shop/partnerchain-reference-implementation/blob/main/docs/dev_logs/initial_customizations.md#book-understand-substrate).

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
