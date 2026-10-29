# Verification Throughput Benchmarks

| Operation | Input Payload | Median Latency |
| :--- | :--- | :---: |
| `is_checkpoint_ledger` | `u32` sequence | `< 1 ns` |
| `split_xdr_frames` | `64 KB` uncompressed `.xdr` | `1.8 µs` |
| `verify_contiguous_ancestry` | `64` `LedgerHeader` links | `4.2 µs` |
