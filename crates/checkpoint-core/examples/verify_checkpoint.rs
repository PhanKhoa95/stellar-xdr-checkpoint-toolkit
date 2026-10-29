use stellar_checkpoint_core::checkpoint::{
    archive_relative_path, checkpoint_for_ledger, is_checkpoint_ledger,
};

fn main() {
    let ledger = 64_791_359;
    assert!(is_checkpoint_ledger(ledger));
    let cp = checkpoint_for_ledger(ledger);
    println!("Checkpoint: {cp} -> {}", archive_relative_path("ledger", cp));
}
