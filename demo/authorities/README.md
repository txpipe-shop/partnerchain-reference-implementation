# Authorities Setup

Authorities in a blockchain participate in the consensus process. In a Substrate-based chain, these authorities are identified via Ed25519 and Sr25519 public keys. In this document we'll briefly explain how to make a custom implementation for the authorities setup and how you can use the implementation we give.

## Runtime APIs

Runtime APIs are traits that are implemented in the runtime and provide both a runtime-side implementation and a client-side API for the node to interact with. There are required runtime APIs like Core, but there can be other user-defined or standard APIs too. You can provide user-defined implementations for the standard APIs too, allowing for an even more customizable runtime.

For the authorities setup, there are two Runtime APIs that are relevant: `sp_consensus_aura::AuraApi<Block, AuraId>` and `sp_consensus_grandpa::GrandpaApi<Block>`. Both of which declare function `authorities()`, from where the current set of authorities is read.
These return a list of `AuraId` or `GrandpaId` respectively.

To customize the logic behind the choice of authorities, a function must be implemented that returns the list of authorities as described. This function must be called from within the runtime so it must compile to WASM. This is important to keep in mind because we can only use non-std packages for the implementation. These are the calls to the functions in the runtime:

https://github.com/txpipe-shop/griffin_partnerchains/blob/02ef9d4246a37856aaa3dc73b05eed01b12fddab/runtime/src/lib.rs#L227-L229

https://github.com/txpipe-shop/griffin_partnerchains/blob/02ef9d4246a37856aaa3dc73b05eed01b12fddab/runtime/src/lib.rs#L233-L235

Here, we have the calls to our custom functions, but our implementation can be replaced in the functions' file or if you wish to create a new module, import it and replace the call. 

We provide an implementation for a function that reads the authorities from a Griffin UTxO, which can be used as an example.

## Demo implementation

For Griffin, we have implemented a novel way to set up the authorities, allowing us to take advantage of the EUTxO model. In this new implementation, the authorities' public keys are read from the datum of a UTxO. This UTxO is initially created in the genesis of the chain, it is paid to an arbitrary address with a token that should ideally be an NFT.
>Since in the genesis any token can be minted, it is not possible for it to be an NFT. So there is room for improvements upon the consumption of this UTxO and ensuring that the new UTxO has the necessary security measures. This is something to keep in mind for the creation of the authority set.

The authorities in the datum are expected to have the following shape:
```json
[
    {
        "pubkey": "0x...",
        "aura_pubkey": "0x...",
        "gran_pubkey": "0x..."
    },
    .
    .
    .    
]
```
where each item is comprised of the keys for one node. The first key is not used in this context.

#### 1. Collect public keys

The first step to the setup is collecting the public keys of all the desired authorities. These might be custom keys created by each node, or they can simply be the public keys of the predefined Substrate accounts.

#### 2. Encode

After obtaining all the keys, encode the list into PlutusData to obtain the CBOR that will go in the datum of the UTxO.

#### 3. Include in genesis

The genesis file can vary as it includes the initial set of UTxOs for the blockchain. As we mentioned the authorities UTxO can be paid to any address, and with a unique token, and it is up to the chain builder to ensure that this is correctly set. Here is an example UTxO with the encoded authorities list as the datum:
```json
{
    "address": "0000000000000000000000000000000000000000000000000000000000",
    "coin": 314150000,
    "value": [
            {
                "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                "assets": [ ["Authorities", 1]]
            }
            ],
    "datum": "9FD879809F9F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC5820B4952B521643E21F4BBEF391CCD0B2FA087C72D2CA14EFDC33DA24BFCC83425C5820E8A9E7E1434F5702D06C48E704EF9DA39BB5C9D62D228D377DDF89AE95173FEBFFFF00FF"
}
```
Ideally it is paid to a script address that makes some validation upon its spending. But for the example any address is enough.

#### 4. Modify authorities identifiers

The UTxO with the authorities set can be owned by any address and include any token, as long as it is configured accordingly in the [authorities crate](../authorities/src/lib.rs). Here we have three constants:
```rust
const AUTHORITIES_ADDRESS: &[u8] = &hex!("0000000000000000000000000000000000000000000000000000000000");
pub const RAW_AUTHORITIES_TOKEN_NAME: &str = "Authorities";
pub const RAW_AUTHORITIES_POLICY_ID: &str = "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005";
```
that need to match the corresponding details in the genesis configuration. If these fields are modified you will need to re-build the project (`cargo build`).

After this configuration is finished, the blockchain will be ready to read the authorities from the genesis!
