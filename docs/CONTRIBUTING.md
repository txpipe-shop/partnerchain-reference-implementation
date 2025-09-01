# Contributing

Thanks for considering contributing and help us on creating the partner chain reference implementation!

The best way to contribute right now is to try things out and provide feedback,
but we also accept contributions to the documentation and obviously to the
code itself.

This document contains guidelines to help you get started and how to make sure
your contribution gets accepted.

## Your first contribution

Contributing to the documentation, reporting bugs or proposing features are awesome ways to get started.

### Documentation

Should you would want to extend the documentation or
find some errors, please file an issue pointing to the mistake or even better,
create a pull request with the changes directly!

### Bug reports

[Submit an issue](https://github.com/txpipe-shop/partnerchain-reference-implementation/issues/new) using the "Bug report" template.

For bug reports, it's very important to explain

- what version you used,
- steps to reproduce (or steps you took),
- what behavior you saw (ideally supported by logs), and
- what behavior you expected.

### Feature ideas

Feature ideas are precursors to high-level features items, which will be
discussed and fleshed out to ideally become items on our feature roadmap.

You can [submit an
issue](https://github.com/txpipe-shop/partnerchain-reference-implementation/issues/new/choose).

We expect a description of

- why you (or the user) need/want something (e.g. problem, challenge, pain, benefit), and
- what this is roughly about (e.g. description of a new API endpoint or message format).

Note that we do NOT require a detailed technical description, but are much more
interested in _why_ a feature is needed. This also helps in understanding the
relevance and ultimately the priority of such an item.

## Making changes

When contributing code, it helps to have discussed the rationale and (ideally)
how something is implemented in a feature idea or bug ticket beforehand.

### Building & Testing

The general Rust installation instructions for any Polkadot-sdk based project are [here](https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/). There might be some extra dependencies needed depending on your system, so make sure to check the instructions thoroughly.

Additional targets and components necessary are: `wasm32-unknown-unknown` and `rust-src`, which can be installed by running:
```bash
rustup target add wasm32-unknown-unknown --toolchain stable-x86_64-unknown-linux-gnu
```
and
```bash
rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
```
respectively.

The running instructions are detailed [here](../README.md/).

Alternatively, you can use `docker` to build and run. But keep in mind that this might take a while.

#### Coding style

Using `cargo fmt` and `cargo clippy` is necessary to ensure a consistent coding style.

### Creating a pull request

Thank you for contributing your changes by opening a pull requests! To get
something merged we usually require:

- Description of the changes - if your commit messages are great, this is less important
- Quality of changes is ensured - through new or updated automated tests in [GitHub Actions](https://github.com/txpipe-shop/partnerchain-reference-implementation/actions)
- Change is related to an issue, feature (idea) or bug report - ideally discussed beforehand
- Well-scoped - we prefer multiple PRs, rather than a big one
