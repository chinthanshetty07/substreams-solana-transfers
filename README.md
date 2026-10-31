# Solana Transfers Substreams

## My Thinking Process

> Full version: [MY_THINKING_PROCESS.md](MY_THINKING_PROCESS.md)

**Understanding the problem.** I'd never used blockchain, Rust or Substreams. I boiled the task down to: *for 6 Solana blocks, find every SOL transfer and print sender, receiver and amount.* The sample amount pointed to native SOL in lamports.

**Concepts I learned.**
- A block holds transactions, and a transaction holds instructions.
- SOL moves through the built-in System Program.
- Amounts are in lamports. Addresses are base58.
- A transfer can hide inside another program's call (an inner instruction).
- Substreams runs my Rust code (compiled to WebAssembly) on each block. It is packaged as an `.spkg`.

**Subproblems.** Install the tools → get an API key → set up the project → decode transfers → match the exact format → run the 6 blocks → verify. I used Claude to learn each piece and plan before coding.

**Decisions.**
- Native SOL `Transfer` instructions only.
- Successful transactions only.
- Inner instructions included, 0-lamport transfers kept.
- Latest official libraries.
- Exact formatting done in `run.sh`.

**Problems and fixes.**
- **API keys rejected everywhere.** I tested each provider and key prefix, which proved the keys were bad. The Graph Market blocks sign-ups in my region, so I switched to Pinax.
- **New library version not in any tutorial.** I read the official changelogs and source.
- **Big numbers printed as strings, and zero amounts printed as `null`.** I fixed both in `run.sh`.
- **The script ignored my endpoint.** It was a one-line ordering bug, now fixed.

**Verification.** 1,564 transfers. All 6 blocks match Solana's own RPC exactly.

---

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
(207 · 181 · 412 · 207 · 155 · 402). Every line matches the required format, and all six
blocks were cross-checked against Solana RPC `getBlock`: every block matches exactly
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
