# Formal Checkpoint & Ancestry Verification Specification

## 1. Checkpoint Sequence Alignment (`INV-CHK-01`)

Every Stellar History Archive checkpoint sequence $c$ satisfies:

$$(c + 1) \equiv 0 \pmod{64}, \quad c \ge 63$$

Any request for an unaligned ledger sequence $s$ must be rejected with `400 Bad Request` and include the containing checkpoint:

$$\text{checkpoint}(s) = \left\lfloor \frac{s + 64}{64} \right\rfloor \times 64 - 1$$

## 2. Contiguous Ancestry Hash Chain (`INV-ANC-02`)

Given an ordered sequence of `LedgerHeaderHistoryEntry` records $H_0, H_1, \dots, H_n$:

1. For all $i \in [1, n]$: $\text{seq}(H_i) = \text{seq}(H_{i-1}) + 1$.
2. For all $i \in [1, n]$: $\text{prev\_hash}(H_i) = \text{SHA256}(\text{XDR}(H_{i-1}.\text{header}))$.
