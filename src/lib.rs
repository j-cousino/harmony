pub mod index;
pub mod blob;

use std::io::Read;
use sha2::{Sha256, Digest};

/// SHA256 Digest of an empty stream. 0 bytes hashed. 
const _EMPTY_STREAM_DIGEST: &str = 
    r"E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855";

/// Return the SHA256 digest of a stream and a string of 32 hex digits
/// 
/// usage:
/// 
/// let mut input = std::fs::File::open(path).unwrap();
/// let hex_string = stream_to_sha256_hex_string(&mut input);
pub fn stream_to_sha256_hex_string<R: ?Sized>(reader: &mut R) -> String 
where 
    R: Read,
{
    let mut hasher = Sha256::new();
    let _ = std::io::copy( reader, &mut hasher );
    hasher.finalize().iter().map(|f| format!("{:02X}", f)).collect()
}