use std::io::{Error, ErrorKind};
use binary_reader::BinaryReader;
use crate::write::write::Write;
use crate::read::read::Read;

// The regulation and the zero padding that follows it always add up to this
// much. The total has been stable across game patches, but the boundary
// between the two moves every time the regulation itself grows (e.g. 1.17),
// so it is located by scanning rather than hardcoded.
pub(crate) const REGULATION_BLOCK_SIZE: usize = 0x240000;

/// Split a regulation+padding block at the end of the real data.
/// The block is zero filled after the regulation, so the trailing run of
/// zeros marks where the data ended. Rounding up to a whole AES block keeps
/// decryption aligned, and also takes back any zero bytes the ciphertext
/// happened to end on.
pub(crate) fn split_regulation_block(block: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let data_len = block.iter().rposition(|byte| *byte != 0).map_or(0, |i| i + 1);
    let regulation_len = data_len.next_multiple_of(0x10).min(block.len());
    (block[..regulation_len].to_vec(), block[regulation_len..].to_vec())
}

pub struct UserData11 {
    unk: [u8;0x10],
    pub regulation: Vec<u8>,
    rest: Vec<u8>,
}

impl Default for UserData11 {
    fn default() -> Self {
        Self { 
            unk: Default::default(), 
            regulation: vec![0; REGULATION_BLOCK_SIZE],
            rest: Vec::new()
        }
    }
}

impl Read for UserData11 {
    fn read(br: &mut BinaryReader) -> Result<UserData11, Error> {
        let mut user_data_11 = UserData11::default();
        user_data_11.unk.copy_from_slice(br.read_bytes(0x10)?);

        let block = br.read_bytes(REGULATION_BLOCK_SIZE)?;
        let (regulation, rest) = split_regulation_block(block);
        if regulation.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "regulation data not found"));
        }
        user_data_11.regulation = regulation;
        user_data_11.rest = rest;
        Ok(user_data_11)
    }
}

impl Write for UserData11 {
    fn write(&self) -> Result<Vec<u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.unk);
        bytes.extend(self.regulation.to_vec());
        bytes.extend(self.rest.to_vec());
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_finds_scanned_boundary() {
        // Data ending mid AES block: boundary rounds up to 0x70.
        let mut block = vec![0u8; 0x100];
        block[..0x64].fill(0x9F);
        let (regulation, rest) = split_regulation_block(&block);
        assert_eq!(regulation.len(), 0x70);
        assert_eq!(rest.len(), 0x100 - 0x70);
        assert_eq!(regulation[..0x64], block[..0x64]);
    }

    #[test]
    fn split_keeps_block_total() {
        // Measured 1.17 PS4 layout: regulation 0x1F3720, rest 0x4C8E0.
        let mut block = vec![0u8; REGULATION_BLOCK_SIZE];
        block[..0x1F3720].fill(0x9F);
        let (regulation, rest) = split_regulation_block(&block);
        assert_eq!(regulation.len(), 0x1F3720);
        assert_eq!(rest.len(), 0x4C8E0);
        assert!(rest.iter().all(|b| *b == 0));
    }

    #[test]
    fn split_empty_block_yields_no_regulation() {
        let block = vec![0u8; 0x100];
        let (regulation, rest) = split_regulation_block(&block);
        assert!(regulation.is_empty());
        assert_eq!(rest.len(), 0x100);
    }

    // Run with: ER_TEST_SAVE=/path/to/memory.dat cargo test -- --ignored
    // Loading a 1.17 save used to panic in UserData11::read (assert on rest[0]).
    #[test]
    #[ignore]
    fn ps4_save_loads_and_preserves_user_data_11() {
        use crate::write::write::Write as WriteTrait;
        let path = std::path::PathBuf::from(
            std::env::var("ER_TEST_SAVE").expect("ER_TEST_SAVE must point at a PS4 memory.dat"),
        );
        let original = std::fs::read(&path).expect("read original save");
        let save = crate::save::save::save::Save::from_path(&path).expect("load save");
        let written = save.write().expect("write save");
        assert_eq!(written.len(), original.len());
        // UserData11 (unk + regulation + padding) is the tail of a PS4 save.
        const USER_DATA_11_OFFSET: usize = 0x1960070;
        assert_eq!(written[USER_DATA_11_OFFSET..], original[USER_DATA_11_OFFSET..]);
        // NOTE: save-slot tails are still zero-padded on write (pre-existing
        // lossy behavior, fixed upstream by PR #100 which is out of scope
        // here), so a full byte-compare is intentionally not asserted.
    }
}
