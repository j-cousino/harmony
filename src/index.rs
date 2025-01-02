use std::collections::{BTreeMap, btree_map::Iter};
use std::io::Write;
use std::path::{PathBuf, Path};
use ignore::Walk;
use serde::{Serialize,Deserialize};
use super::*;

/// A light wrapper around a hashmap
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    inner: BTreeMap<PathBuf, String>,
}

impl Index {
    pub fn new<P: AsRef<Path>>(path: P) -> Index {
        let mut index = Index{ inner: BTreeMap::new(), };
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

    pub fn save<P: AsRef<Path>>(&self,  path: P) -> anyhow::Result<usize> {
        let encoded = bincode::serialize(self)?;
        Ok(std::fs::File::create(path).unwrap().write(&encoded)?)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> anyhow::Result<Index> {
        let mut encoded= Vec::<u8>::new();
        let _ = std::fs::File::open(path).unwrap().read_to_end(&mut encoded);
        let index: Index = bincode::deserialize(&encoded)?;
        Ok(index)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
