use sha2::{Digest, Sha256};
use std::io::{self, Read};

const BUFFER: usize = 64 * 1024;

pub fn hash_limited<R: Read>(reader: &mut R, allowance: u64) -> io::Result<(String, u64, bool)> {
    let mut hash = Sha256::new();
    let mut buf = [0u8; BUFFER];
    let mut read = 0u64;
    loop {
        let remaining = allowance.saturating_sub(read);
        let cap = remaining.saturating_add(1).min(BUFFER as u64) as usize;
        let n = reader.read(&mut buf[..cap])?;
        if n == 0 {
            break;
        }
        read = read
            .checked_add(n as u64)
            .ok_or_else(|| io::Error::other("size overflow"))?;
        if read > allowance {
            return Ok((String::new(), read, true));
        }
        hash.update(&buf[..n]);
    }
    Ok((format!("{:x}", hash.finalize()), read, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn injected_growth_is_stopped_at_allowance_plus_one() {
        let mut reader = Cursor::new(vec![b'x'; 12]);
        let (_, count, exceeded) = hash_limited(&mut reader, 5).unwrap();
        assert!(exceeded);
        assert_eq!(count, 6);
        assert_eq!(reader.position(), 6);
    }
}
