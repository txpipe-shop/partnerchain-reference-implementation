# Griffin-RPC

Extension of the RPC system to include Griffin-specific queries.

## Cardano_UTxORPC

### _Method name_: `submit_tx`

Tries to submit a transaction in CBOR format.

Usage example:

```bash
curl -H "Content-Type: application/json" -d '{ "id":1, "jsonrpc":"2.0", "method":"cardano_utxorpc_submit_tx", "params":["84A300818258208A238075B88ABA304618A8795EE1CD2342B181306D371D7233755CC29D1ACD36000182A200581D614FDF13C0AABB2C2E6DF7A0AC0F5CB5AAABCA448AF8287E54681273DD011A00989680A200581D6101E6301758A6BADFAB05035CFFC8E3438B3AFF2A4EDC6544B47329C401821A121EAC00A1581C0298AA99F95E2FE0A0132A6BB794261FB7E7B0D988215DA2F2DE2005A246746F6B656E411A102721C046746F6B656E421A42F87D890200A100818258207B155093789404780735F4501C576E9F6E2B0A486CDEC70E03E1EF8B9EF992745840CBE970F4487A48030A4DFD7E413030F5FE0E0E3D75431B89C34E0DE1903DC3746692A716511E43D9DE1773BD02C693AFBCCBA324FD4F134852F35E00D9192200F5F7" ] }' http://127.0.0.1:9944
```

## UTxO RPC

### _Method name_: `get_utxo`

Tries to obtain a UTxO from its output reference. Throws an error if such UTxO does not exist.

Usage example:

```bash
 curl -H "Content-Type: application/json" -d '{ "id":1, "jsonrpc":"2.0", "method":"utxorpc_get_utxo", "params":["8a238075b88aba304618a8795ee1cd2342b181306d371d7233755cc29d1acd3600000000" ] }' http://127.0.0.1:9944
```

### _Method name_: `get_utxo_by_address`

Obtains the list of UTxOs belonging to an address. Returns an empty list if no UTxOs are found for such address.

Usage example:

```bash
 curl -H "Content-Type: application/json" -d '{ "id":1, "jsonrpc":"2.0", "method":"utxorpc_get_utxo_by_address", "params":["6101e6301758a6badfab05035cffc8e3438b3aff2a4edc6544b47329c4" ] }' http://127.0.0.1:9944
```

### _Method name_: `get_utxo_with_asset`

Obtains the list of UTxOs that contain a certain Asset. Returns an empty list if no UTxOs are found. Throws an error if the policy is invalid.

Usage example:

```bash
 curl -H "Content-Type: application/json" -d '{ "id":1, "jsonrpc":"2.0", "method":"utxorpc_get_utxo_with_asset", "params":["tokenA", "0298aa99f95e2fe0a0132a6bb794261fb7e7b0d988215da2f2de2005"] }' http://127.0.0.1:9944
```
