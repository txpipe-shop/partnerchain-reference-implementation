# One-Shot Minting Policy

This script takes as parameter a UTxO reference (tx hash and index) and allows the minting/burning of an NFT. It checks that:

- the minted/burned amount is exactly 1.
- if minting, the UTxO passed as a parameter is consumed.
