# Solana Transfers Substreams

A Rust Substreams module that extracts native SOL transfers from Solana blocks
**385870151 – 385870156** and prints one JSON object per transfer:

```json
{"from": "Eq1c3uNugn3FRpJ99K2q1ZyQrora3jntUqvkMnGroKkB", "to": "4XpSNfoNfireSgdg8hY6Ng3dWrHjewZhmCancypjbcCN", "amount": 325253}
```

## How it works

`map_transfers` reads each `sf.solana.type.v1.Block` and walks every instruction,
including inner (CPI) instructions, of every **successful** transaction. It keeps
System Program (`11111111111111111111111111111111`) instructions whose data starts with
discriminator `2` (`Transfer`), then reads:

| Field    | Source                                  |
|----------|-----------------------------------------|
| `from`   | instruction account 0, base58           |
| `to`     | instruction account 1, base58           |
| `amount` | `u64` little-endian at data bytes 4..12, in lamports |

The module emits `sol.transfers.v1.Transfers` (one message per block, one entry per
transfer). Protobuf JSON encodes `uint64` as a string, so `run.sh` reformats each entry
into the exact line format above with `amount` as an integer.

## Prerequisites

- [Rust](https://rustup.rs) with the `wasm32-unknown-unknown` target (pinned in `rust-toolchain.toml`)
- [Substreams CLI](https://docs.substreams.dev/how-to-guides/installing-the-cli): `brew install streamingfast/tap/substreams`
- [buf](https://buf.build/docs/installation) (protobuf code generation) and `jq`
- A Substreams API key from [The Graph Market](https://thegraph.market) or [Pinax](https://app.pinax.network)

## Usage

```bash
cp .env.example .env               # set SUBSTREAMS_API_KEY or SUBSTREAMS_API_TOKEN (and endpoint)
./.unifiedhiring/run.sh            # build the .spkg and stream blocks 385870151–385870156
```

Other commands:

```bash
substreams build                   # compile to wasm and package solana-transfers-v0.1.0.spkg
cargo test                         # unit tests for instruction decoding
```

Set `SUBSTREAMS_ENDPOINT` to use a provider other than `mainnet.sol.streamingfast.io:443`
(this run used Pinax, `solana.substreams.pinax.network:443`).

## Result

`output/transfers.jsonl` holds the full run: **1,564 transfers** across the six blocks
(207 · 181 · 412 · 207 · 155 · 402). Every line matches the required format, and block
385870151 was cross-checked against Solana RPC `getBlock`: the same 207 transfers
(sender, recipient, amount).

Protobuf JSON omits zero values, so 0-lamport transfers (2 in this range) are printed
as `"amount": 0` by `run.sh`.

## Layout

```
proto/transfers.proto   output schema (sol.transfers.v1)
src/lib.rs              map_transfers handler and decoding
src/pb/                 generated bindings (buf generate / substreams build)
substreams.yaml         Substreams manifest
.unifiedhiring/run.sh   build + run + format
output/transfers.jsonl  output for blocks 385870151–385870156
```
