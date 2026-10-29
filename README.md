# Stellar XDR Checkpoint & Ancestry Toolkit

Deterministic utilities in **Rust** and **Go** for verifying Stellar History Archive checkpoint alignment (`(seq + 1) % 64 == 0`), decoding RFC 4506 XDR frame streams, and validating `previous_ledger_hash` ancestry chains.

## Core Capabilities

| Module | Language | Responsibility |
| :--- | :---: | :--- |
| `crates/checkpoint-core` | Rust | Checkpoint boundary arithmetic, hex path resolution, and RFC 4506 frame parsing |
| `crates/ancestry-verifier` | Rust / Wasm | Deterministic SHA-256 `LedgerHeaderHistoryEntry` chain verification |
| `pkg/checkpointhttp` | Go | Immutable `Cache-Control` and strong `ETag` (`If-None-Match`) handlers for `.xdr.gz` streams |
| `docs/specification.md` | Markdown | Formal invariants for Stellar History Archive checkpoint verification |

## License

MIT
