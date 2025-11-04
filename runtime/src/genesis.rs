//! Helper module to build a genesis configuration for the runtime.

#[cfg(feature = "std")]
pub use super::WASM_BINARY;
use alloc::string::String;
use griffin_core::genesis::config_builder::GenesisConfig;
use serde_json;

/// The default genesis. It can be replaced by a custom one by providing the
/// node with an analogous JSON file through the `--chain` flag
pub const GENESIS_DEFAULT_JSON: &str = r#"
{
    "zero_time": 1747081100000,
    "zero_slot": 0,
    "slot_length": 3000,
    "outputs": [
        {
            "address": "6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4",
            "coin": 314000000,
            "value": [
                    {
                        "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                        "assets": [ ["tokenA", 271000000], ["tokenB", 1123581321] ]
                    },
                    {
                        "policy": "7ba97fb6e48018ef131dd08916939350c0ce7050534f8e51b5e0e3a4",
                        "assets": [ ["PILOT0", 1], ["PILOT1", 1] ]
                    }
                    ],
            "datum": "820080"
        },
        {
            "address": "70bac1753d5f7e3609c92776371fd0eafa753889e5712858f48fb83981",
            "coin": 500000000,
            "value": [
                    {
                        "policy": "516238dd0a79bac4bebe041c44bad8bf880d74720733d2fc0d255d28",
                        "assets": [ ["asteriaAdmin", 1] ]
                    }
                    ],
            "datum": "D8799F00581C7BA97FB6E48018EF131DD08916939350C0CE7050534F8E51B5E0E3A4FF"
        },
        {
            "address": "707ba97fb6e48018ef131dd08916939350c0ce7050534f8e51b5e0e3a4",
            "coin": 2000,
            "value": [
                    {
                        "policy": "6a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
                        "assets": [ ["FUEL", 25] ]
                    },
                    {
                        "policy": "7ba97fb6e48018ef131dd08916939350c0ce7050534f8e51b5e0e3a4",
                        "assets": [ ["SHIP1", 1] ]
                    }
                    ],
            "datum": "D8799F00004553484950314650494C4F54311B00000196C625FAE0FF"
        },
        {
            "address": "706a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
            "coin": 2000,
            "value": [
                    {
                        "policy": "6a25ad5476105ac4a3784769cb93f92fd67a11932ef9a65a61abd1d6",
                        "assets": [ ["FUEL", 80] ]
                    },
                    {
                        "policy": "516238dd0a79bac4bebe041c44bad8bf880d74720733d2fc0d255d28",
                        "assets": [ ["asteriaAdmin", 1] ]
                    }
                    ],
            "datum": "d8799f2703581c7ba97fb6e48018ef131dd08916939350c0ce7050534f8e51b5e0e3a4ff"
        },
        {
            "address": "0000000000000000000000000000000000000000000000000000000000",
            "coin": 314150000,
            "value": [
                    {
                        "policy": "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005",
                        "assets": [ ["Authorities", 300000000]]
                    }
                    ],
            "datum": "9FD879809F9F5821022A009DD29E31A1573BF90EBE5979D496B3C45CC898F0E39BF16563F4435F5BAC5820D43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D582088DC3417D5058EC4B4503E0C12EA1A0A89BE200FE98922423D4334014FA6B0EEFFFF00FF"
        }
    ]
}
"#;

/// This function builds the genesis configuration from the provided json string.
/// It is called by the `ChainSpec::build` method.
///
/// If a custom genesis is not provided, [GENESIS_DEFAULT_JSON] is used.
pub fn get_genesis_config(genesis_json: String) -> GenesisConfig {
    let mut json_data: &str = GENESIS_DEFAULT_JSON;
    if !genesis_json.is_empty() {
        json_data = &genesis_json;
    };

    match serde_json::from_str(json_data) {
        Err(e) => panic!("Error: {e}\nJSON data: {json_data}"),
        Ok(v) => v,
    }
}
