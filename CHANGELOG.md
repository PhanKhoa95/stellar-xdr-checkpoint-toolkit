# Changelog

## [0.1.0] - 2026-10-29

### Added
- `stellar-checkpoint-core`: 64-ledger checkpoint alignment, History Archive path builder, RFC 4506 XDR frame splitter, and contiguous `previous_ledger_hash` ancestry verifier.
- `pkg/checkpointhttp`: Go HTTP handler serving `.xdr.gz` checkpoint files with strong `ETag` (`304 Not Modified`) and immutable `Cache-Control`.
