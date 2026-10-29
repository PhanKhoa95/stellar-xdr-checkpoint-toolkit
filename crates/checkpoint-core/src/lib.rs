pub mod ancestry;
pub mod checkpoint;
pub mod frame;

#[cfg(test)]
mod tests {
    use super::ancestry::{verify_contiguous_ancestry, HeaderLink};
    use super::checkpoint::{archive_relative_path, checkpoint_for_ledger, is_checkpoint_ledger};
    use super::frame::split_xdr_frames;

    #[test]
    fn checkpoint_boundaries_match_stellar_archive_spec() {
        assert!(!is_checkpoint_ledger(0));
        assert!(is_checkpoint_ledger(63));
        assert!(!is_checkpoint_ledger(64));
        assert!(is_checkpoint_ledger(64_791_359));
        assert_eq!(checkpoint_for_ledger(64_791_300), 64_791_359);
        assert_eq!(
            archive_relative_path("ledger", 0x03dca33f),
            "ledger/03/dc/a3/ledger-03dca33f.xdr.gz"
        );
    }

    #[test]
    fn rfc4506_frame_splitter_extracts_records() {
        let payload = [0x80, 0x00, 0x00, 0x03, b'x', b'd', b'r'];
        let frames = split_xdr_frames(&payload).expect("valid frame");
        assert_eq!(frames, vec![&b"xdr"[..]]);
    }

    #[test]
    fn ancestry_verifier_validates_hash_links() {
        let h1 = HeaderLink {
            ledger_seq: 100,
            header_hash: [1u8; 32],
            previous_ledger_hash: [0u8; 32],
        };
        let h2 = HeaderLink {
            ledger_seq: 101,
            header_hash: [2u8; 32],
            previous_ledger_hash: [1u8; 32],
        };
        assert_eq!(verify_contiguous_ancestry(&[h1, h2]), Ok((100, 101)));
    }
}
