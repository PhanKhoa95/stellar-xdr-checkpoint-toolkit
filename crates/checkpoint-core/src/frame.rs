//! RFC 4506 Record Marking Standard frame parser for Stellar `.xdr` archive files.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameError {
    TruncatedHeader,
    MissingEndOfRecordBit(u32),
    TruncatedPayload { expected: usize, available: usize },
}

/// Iterates over RFC 4506 record-marked frames in an uncompressed XDR buffer.
pub fn split_xdr_frames(mut buf: &[u8]) -> Result<Vec<&[u8]>, FrameError> {
    let mut frames = Vec::new();
    while !buf.is_empty() {
        if buf.len() < 4 {
            return Err(FrameError::TruncatedHeader);
        }
        let raw_hdr = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if (raw_hdr & 0x8000_0000) == 0 {
            return Err(FrameError::MissingEndOfRecordBit(raw_hdr));
        }
        let frame_len = (raw_hdr & 0x7fff_ffff) as usize;
        buf = &buf[4..];
        if buf.len() < frame_len {
            return Err(FrameError::TruncatedPayload {
                expected: frame_len,
                available: buf.len(),
            });
        }
        frames.push(&buf[..frame_len]);
        buf = &buf[frame_len..];
    }
    Ok(frames)
}
