#!/usr/bin/env bash
# Builds the Substreams package and prints every native SOL transfer in blocks
# 385870151-385870156 as one JSON object per line, in the format task.md requires.
set -euo pipefail

cd "$(dirname "$0")/.."

START_BLOCK=385870151
STOP_BLOCK=+6 # exclusive: 385870151..385870156
ENDPOINT="${SUBSTREAMS_ENDPOINT:-mainnet.sol.streamingfast.io:443}"

# Credentials: an API key in .env, or a JWT written to .substreams.env by `substreams auth`.
for env_file in .env .substreams.env; do
  if [[ -f "$env_file" ]]; then
    set -a; source "$env_file"; set +a
  fi
done
if [[ -n "${SUBSTREAMS_API_TOKEN:-}" ]]; then
  unset SUBSTREAMS_API_KEY # prefer the JWT when both are present
elif [[ -z "${SUBSTREAMS_API_KEY:-}" ]]; then
  echo "error: set SUBSTREAMS_API_KEY in .env or run 'substreams auth'" >&2
  exit 1
fi

substreams build >&2

substreams run solana-transfers-v0.1.0.spkg map_transfers \
  -e "$ENDPOINT" -s "$START_BLOCK" -t "$STOP_BLOCK" -o jsonl \
  | jq -r '.["@data"].transfers[]? | "{\"from\": \"\(.from)\", \"to\": \"\(.to)\", \"amount\": \(.amount)}"'
