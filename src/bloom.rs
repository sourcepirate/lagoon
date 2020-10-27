use bit_vec::BitVec;
use fasthash::murmur3::hash128_with_seed;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum BloomError {
    DoesNotExist,
    DuplicateError,
    MaxEntryReached,
    Other,
}

/// Represent a collection of bloom nodes
pub struct BloomCollection {
    _inner: BTreeMap<String, Arc<Mutex<BloomNode>>>,
}

/// Represents the bloom filter node.
pub struct BloomNode {
    /// Represents the bit vector for the bloom filter
    /// Typical bit vector contains 1's and 0's on a seqential manner
    inner: BitVec,
    /// M represents the number of bits that should be on the bit vector
    m: usize,
    /// N represents the maximum of number of elements that can be store.
    /// n can also be derived by the optimal probability distruction respresented
    /// with respect to k where k = (m / n) * ln 2
    /// n = (m / k) * ln 2
    n: usize,
    /// K denotes the number of hash functions to be applied.
    k: usize,
    /// current number of elements
    current: usize,
}

impl BloomNode {
    pub fn new(m: usize, k: usize) -> Self {
        let n: usize = ((m / k) as f64 * 2_f64.ln()) as usize;
        BloomNode {
            inner: BitVec::from_elem(m as usize, false),
            m,
            n,
            k,
            current: 0,
        }
    }

    pub fn has<T>(&self, el: T) -> bool
    where
        T: AsRef<[u8]>,
    {
        for i in 0..self.k {
            let hash_val = hash128_with_seed(&el, i as u32);
            if let Some(bit_val) = self.inner.get(hash_val as usize % BloomNode::max_bits()) {
                if !bit_val {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn set<T>(&mut self, el: T) -> Result<(), BloomError>
    where
        T: AsRef<[u8]>,
    {
        for i in 0..self.k {
            let hash_val = hash128_with_seed(&el, i as u32);
            let position = hash_val as usize % BloomNode::max_bits();
            if (self.current + 1) > self.n {
                return Err(BloomError::MaxEntryReached);
            }
            self.inner.set(position, true);
        }
        self.current += 1;
        Ok(())
    }

    pub fn bit_size(&self) -> usize {
        self.m
    }

    #[inline]
    pub fn max_bits() -> usize {
        if cfg!(feature = "16") {
            return u16::MAX as usize;
        } else {
            return u32::MAX as usize;
        }
    }

    #[inline]
    pub fn max_hash() -> usize {
        if cfg!(feature = "16") {
            return (u8::MAX / 4) as usize;
        } else {
            return u8::MAX as usize;
        }
    }
}

impl BloomCollection {
    pub fn new() -> Self {
        BloomCollection {
            _inner: BTreeMap::new(),
        }
    }

    pub fn create(&mut self, collection: String, m: usize, k: usize) -> Result<(), BloomError> {
        let tex: Mutex<BloomNode> = Mutex::new(BloomNode::new(m, k));
        match self._inner.get(&collection) {
            Some(_) => Err(BloomError::DuplicateError),
            None => {
                self._inner.insert(collection, Arc::new(tex));
                return Ok(());
            }
        }
    }

    pub fn exist(&self, collection: String, val: String) -> Result<bool, BloomError> {
        let tex = self._inner.get(&collection);
        match tex {
            Some(tr) => {
                let data = tr.clone();
                let guard = data.lock().unwrap();
                Ok(guard.has(val))
            }
            None => Err(BloomError::DoesNotExist),
        }
    }

    pub fn set(&self, collection: String, val: String) -> Result<(), BloomError> {
        let tex = self._inner.get(&collection);
        match tex {
            Some(tr) => {
                let data = tr.clone();
                let mut guard = data.lock().unwrap();
                if guard.has(&val) {
                    Err(BloomError::DuplicateError)
                } else {
                    guard.set(&val)
                }
            }
            None => Err(BloomError::DoesNotExist),
        }
    }

    pub fn delete(&mut self, collection: String) -> Result<(), BloomError> {
        let tex = self._inner.remove(&collection);
        match tex {
            Some(_) => Ok(()),
            None => Err(BloomError::DoesNotExist),
        }
    }

    pub fn has_collection(&mut self, collection: String) -> Result<bool, BloomError> {
        let tex = self._inner.get(&collection);
        match tex {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

impl BloomError {
    pub fn code(&self) -> i64 {
        match self {
            &BloomError::DoesNotExist => 100,
            &BloomError::DuplicateError => 101,
            &BloomError::MaxEntryReached => 102,
            &BloomError::Other => 103,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter_on_collection() {
        let mut filter = BloomCollection::new();
        let collection_name = "hello".to_owned();
        let _ = filter.create(
            collection_name.clone(),
            BloomNode::max_bits(),
            BloomNode::max_hash(),
        );
        for n in 0..1000 {
            let _ = filter.set(collection_name.clone(), n.to_string());
        }
        for i in 0..1000 {
            let flag: bool = filter
                .exist(collection_name.clone(), i.to_string())
                .unwrap();
            assert!(flag);
        }
    }

    #[test]
    fn test_create_and_remove_collection() {
        let mut filter = BloomCollection::new();
        let collection_name = "hello".to_owned();
        let mut flag: bool = filter.has_collection(collection_name.clone()).unwrap();
        assert!(!flag);
        let _ = filter.create(
            collection_name.clone(),
            BloomNode::max_bits(),
            BloomNode::max_hash(),
        );
        flag = filter.has_collection(collection_name.clone()).unwrap();
        assert!(flag);
        let _ = filter.delete(collection_name.clone());
        flag = filter.has_collection(collection_name.clone()).unwrap();
        assert!(!flag);
    }
}
