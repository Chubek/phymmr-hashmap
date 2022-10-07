#![allow(unused)]

use fasthash::metro;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
struct HashMapNode {
    key: Vec<u8>,
    value: Vec<u8>,
    hash_code: u64,
    next: Option<Box<HashMapNode>>,
}

impl HashMapNode {
    pub fn new(key: Vec<u8>, value: Vec<u8>, hash_code: u64, next: Option<Box<Self>>) -> Self {
        Self {
            key,
            value,
            hash_code,
            next,
        }
    }

    pub fn set_key(&mut self, key: Vec<u8>) {
        self.key = key;
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }

    pub fn set_next(&mut self, next: Option<Box<Self>>) {
        self.next = next;
    }
}

#[pyclass]
struct PhyHashMap {
    bucket_array: Vec<Option<Box<HashMapNode>>>,
    num_buckets: isize,
    array_size: usize,
}

#[pymethods]
impl PhyHashMap {
    #[new]
    pub fn new() -> Self {
        let bucket_array = vec![None; 10];
        let num_buckets = 10isize;
        let array_size = 0;

        Self {
            bucket_array,
            num_buckets,
            array_size,
        }
    }

    pub fn len(&self) -> usize {
        self.array_size
    }

    fn hash_key(&self, key: Vec<u8>) -> u64 {
        metro::hash64(key)
    }

    fn get_bucket_index(&self, key: Vec<u8>) -> usize {
        let hashcode = self.hash_key(key);
        let index = hashcode as isize % self.num_buckets;

        match index < 0 {
            true => (index * -1) as usize,
            false => index as usize,
        }
    }

    pub fn remove(&mut self, key_str: String) -> String {
        let key = key_str.as_bytes().to_vec();

        let bucket_index = self.get_bucket_index(key.clone());
        let hash_code = self.hash_key(key.clone());

        let mut head = self.bucket_array.get(bucket_index);

        let mut prev = None;

        match head {
            Some(mut hh) => {
                let mut h = hh.clone();

                while h.clone().is_some() {
                    let mut node = h.clone().unwrap();

                    if node.key == key && node.hash_code == hash_code {
                        break;
                    }

                    prev = Some(node.clone());
                    node.set_next(node.next.clone());

                    h = Some(node);
                }

                if h.is_none() {
                    return String::new();
                }

                self.array_size -= 1;

                if prev.is_some() {
                    let mut prev_node = prev.unwrap();
                    prev_node.set_next(h.clone().unwrap().next);
                    prev = Some(prev_node);
                } else {
                    self.bucket_array[bucket_index] = h.clone().unwrap().next;
                }

                return String::from_utf8(h.unwrap().value).unwrap();
            }
            None => panic!("No such key!"),
        }

        String::new()
    }

    pub fn get(&self, key_str: String) -> String {
        let key = key_str.as_bytes().to_vec();

        let bucket_index = self.get_bucket_index(key.clone());
        let hash_code = self.hash_key(key.clone());

        let head = self.bucket_array.get(bucket_index);

        match head {
            Some(hh) => {
                let mut h = hh.clone();

                while h.is_some() {
                    let node = h.clone().unwrap();

                    if node.key == key && node.hash_code == hash_code {
                        return String::from_utf8(node.value).unwrap();
                    }

                    h = node.next;
                }
            }
            None => panic!("No such key!"),
        }

        String::new()
    }

    pub fn insert(&mut self, key_str: String, value_str: String) {
        let key = key_str.as_bytes().to_vec();
        let value = value_str.as_bytes().to_vec();

        let bucket_index = self.get_bucket_index(key.clone());
        let hash_code = self.hash_key(key.clone());

        let head = self.bucket_array.get(bucket_index);

        if head.is_some() {
            let mut h = head.unwrap().clone();

            while h.is_some() {
                let mut node = h.clone().unwrap();

                if node.key == key && node.hash_code == hash_code {
                    node.value = value;
                    return;
                }

                h = node.next;
            }
        }

        self.array_size += 1;
        let head = self.bucket_array.get(bucket_index);
        let new_node = HashMapNode::new(key, value, hash_code, head.unwrap().clone());
        self.bucket_array[bucket_index] = Some(Box::new(new_node));

        let size_float = self.array_size as f32;

        if 1.0f32 * size_float / self.num_buckets as f32 == 0.7 {
            let temp = self.bucket_array.clone();
            let new_num_buckets = 2 * self.num_buckets;
            self.num_buckets = new_num_buckets;
            let extend = vec![None; new_num_buckets as usize];
            self.bucket_array.extend(extend.into_iter());
            self.array_size = 0;
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn phymmr_hashmap(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
