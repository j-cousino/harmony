use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};

/// Represents a file of any type
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Blob {
    path: PathBuf,
    digest: Vec<u8>,
}

impl Blob {
    /// Creates a new blob object given a path
    /// 
    /// Assumes the path is to an existing file and is readable. otherwise it will panic.
    /// 
    /// usage:
    /// let blob = Blob::new("./cargo.toml");
    /// 
    pub fn new(path: &Path) -> Blob {
        let mut input = std::fs::File::open(path).expect("Could not open the file!");
        let mut hasher = Sha256::new();
        let _ = std::io::copy(&mut input, &mut hasher);
        Blob {
            path: PathBuf::from(&path),
            digest: hasher.finalize().iter().map(|f| f+0).collect(),
        }
    }

    pub fn digest_as_string( &self ) -> String {
        self.digest.iter().map(|f| format!("{:02X}", f)).collect()
    }

    pub fn path_as_string( &self ) -> String {
        self.path.to_string_lossy().into_owned()
    }
}
