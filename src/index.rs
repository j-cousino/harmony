use std::collections::{HashMap, hash_map::Iter};
use std::path::{PathBuf, Path};
use ignore::Walk;
use super::*;

/// A light wrapper around a hashmap
#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    inner: HashMap<PathBuf, String>,
}

impl Index {
    pub fn new<P: AsRef<Path>>(path: P) -> Index {
        let mut index = Index{ inner: HashMap::new(), };
        for result in Walk::new(path) {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        let mut reader = std::fs::File::open(path).unwrap();
                        index.inner.insert(path.to_path_buf(),
                            stream_to_sha256_hex_string(&mut reader));
                    }
                },
                Err(_) => (),
            }
        }
        index 
    }

    pub fn iter(&self) -> Iter<'_, PathBuf, String> {
        self.inner.iter()
    }
}
