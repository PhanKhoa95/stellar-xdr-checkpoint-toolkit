//! Stellar History Archive checkpoint boundary calculations.
//! Checkpoints occur every 64 ledgers at sequences `64*k - 1` (`63, 127, 191, ...`).

/// Returns `true` if `ledger_seq` is a valid Stellar History Archive checkpoint ledger.
#[inline]
pub const fn is_checkpoint_ledger(ledger_seq: u32) -> bool {
    ledger_seq >= 63 && ((ledger_seq as u64 + 1) % 64 == 0)
}

/// Returns the Stellar History Archive checkpoint sequence containing `ledger_seq`.
#[inline]
pub const fn checkpoint_for_ledger(ledger_seq: u32) -> u32 {
    let aligned = ((ledger_seq as u64 + 64) / 64) * 64 - 1;
    aligned as u32
}

/// Formats the canonical History Archive relative path for a category and checkpoint.
pub fn archive_relative_path(category: &str, checkpoint: u32) -> String {
    let b0 = (checkpoint >> 24) & 0xff;
    let b1 = (checkpoint >> 16) & 0xff;
    let b2 = (checkpoint >> 8) & 0xff;
    format!(
        "{category}/{b0:02x}/{b1:02x}/{b2:02x}/{category}-{checkpoint:08x}.xdr.gz"
    )
}
