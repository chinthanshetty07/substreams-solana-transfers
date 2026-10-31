# My Thinking Process

I had never worked with blockchain, Rust or Substreams before this. Here is how I approached it.

## 1. How I understood the problem

- Read task.md twice before touching any code.
- Rewrote it in one line for myself: *"For 6 Solana blocks, find every SOL transfer and print who sent it, who got it, and how much."*
- The output format is fixed. That told me the formatting had to match exactly, down to the spaces.
- The sample amount (325253) was small and had no decimals. That pointed to native SOL, counted in lamports.

## 2. Concepts I had to learn

**Blockchain / Solana**
- A block is a batch of transactions. A transaction holds instructions.
- Each instruction calls a program. Moving SOL is done by the built-in **System Program**.
- Amounts are in **lamports** (1 SOL = 1,000,000,000 lamports).
- Addresses are 32 bytes, shown as **base58** text.
- Programs can call other programs, so a transfer can be hidden inside another instruction (an *inner instruction*).
- Failed transactions are still in the block, but no money moved.

**Substreams**
- A way to stream blockchain data and transform it with your own code.
- My code is a Rust **map module**. It gets one block in and returns my own data out.
- It compiles to WebAssembly and is packaged as an **.spkg** file.
- `substreams.yaml` describes the module. A `.proto` file describes the output shape.
- You need an API key from a provider to stream real data.

## 3. How I split it into smaller problems

1. Install the tools: Rust, the Substreams CLI, `buf`.
2. Get a working API key.
3. Set up a clean Rust project.
4. Decode a transfer from raw instruction bytes.
5. Print it in the exact JSON format.
6. Run it on the 6 blocks.
7. Prove the output is correct.

I used Claude to explain each concept, then made a plan before writing code.

## 4. Decisions I made

- **Only native SOL transfers.** The System Program `Transfer` instruction, as in the task's example.
- **Only successful transactions.** A failed transfer didn't move any SOL.
- **Include inner instructions.** A transfer made by another program is still a real transfer.
- **Keep 0-lamport transfers.** They are still transfer instructions.
- **Use the latest official libraries.** Not older tutorial versions.
- **Keep it small.** One module, one proto, one script. No extra dependencies.
- **Do the exact formatting in `run.sh`.** The Rust module stays clean, typed data.

## 5. Problems I faced and how I solved them

| Problem | What I did |
|---|---|
| My first two API keys were rejected everywhere | Tested them against each provider, and with each key prefix. That proved the keys were the problem, not my code. |
| The Graph Market sign-up is blocked in my region | Didn't wait. Signed up with another provider (Pinax) and used its JWT token. |
| The newest libraries changed how protobuf works, and tutorials were outdated | Read the official changelogs and source code instead of guessing. |
| `substreams build` generated dozens of files I didn't need | Found `excludePaths` in the manifest, so only my own types are generated. |
| Protobuf JSON prints big numbers as strings (`"325253"`) | Reformatted in `run.sh` with `jq`, so `amount` is a plain integer with no rounding. |
| The script ignored my Pinax endpoint | It set the endpoint before loading `.env`. I moved one line. |
| Two transfers printed `"amount": null` | Protobuf leaves out zero values. I defaulted a missing amount to `0`. |
| Tight deadline | Packaged a working submission early as a fallback, then kept improving it. |

## 6. How I know it's correct

- 1,564 transfers across the 6 blocks, and every line matches the required format.
- I cross-checked against Solana's own RPC (`getBlock`). I counted every successful System Program transfer myself and compared sender, receiver and amount for each.
- Unit tests cover decoding: a valid transfer, other instructions, and broken data.

## What I took away

- Break the unknown into small, checkable pieces.
- Read the source when docs fall behind.
- When blocked, find another route instead of waiting.
- Verify against an independent source, not just "it ran".
