//! Deterministic verification of contiguous `previous_ledger_hash` links.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderLink {
    pub ledger_seq: u32,
    pub header_hash: [u8; 32],
    pub previous_ledger_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AncestryError {
    EmptyChain,
    SequenceGap { expected: u32, found: u32 },
    HashMismatch { at_ledger: u32 },
}

/// Verifies that `headers` form an unbroken contiguous hash chain from `first` to `last`.
pub fn verify_contiguous_ancestry(headers: &[HeaderLink]) -> Result<(u32, u32), AncestryError> {
    let first = headers.first().ok_or(AncestryError::EmptyChain)?;
    for window in headers.windows(2) {
        let prev = &window[0];
        let curr = &window[1];
        if curr.ledger_seq != prev.ledger_seq.saturating_add(1) {
            return Err(AncestryError::SequenceGap {
                expected: prev.ledger_seq.saturating_add(1),
                found: curr.ledger_seq,
            });
        }
        if curr.previous_ledger_hash != prev.header_hash {
            return Err(AncestryError::HashMismatch {
                at_ledger: curr.ledger_seq,
            });
        }
    }
    let last = headers.last().expect("non-empty");
    Ok((first.ledger_seq, last.ledger_seq))
}
