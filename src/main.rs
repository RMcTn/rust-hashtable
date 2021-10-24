use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

#[derive(Debug)]
struct HashTable<K: Eq + Clone + Hash + Default, V: Clone + Default> {
    size: usize,
    table: Vec<HashTableEntry<K, V>>,
    current_size: usize,
}

#[derive(Clone, Debug)]
struct HashTableEntry<K: Eq + Clone + Hash + Default, V: Clone + Default> {
    key: K,
    value: V,
    is_alive: bool,
    has_been_used: bool,
}

// impl fmt::Display for HashTable {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (i, entry) in self.table.iter().enumerate() {
//             write!(f, "{}: Key: {} Value: {}", i, entry.key, entry.value)
//         }
//     }
// }

impl<K: Eq + Clone + Hash + Default, V: Clone + Default> HashTable<K, V> {
    fn new(size: usize) -> Self {
        HashTable {
            size,
            table: vec![HashTableEntry { key: Default::default(), value: Default::default(), is_alive: false, has_been_used: false }; size],
            current_size: 0,
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        return match self.find_slot(key) {
            Some(index) => {
                if self.table[index].key == *key && self.table[index].is_alive {
                    Some(&self.table[index].value)
                } else {
                    None
                }
            }
            None => { None }
        };
    }

    /// Returns index of desired key, or index of first unused slot. Index of unused slot means the
    /// key does not exist in the table, since they key would be found before an unused slot.
    /// Table entry for returned index needs to be checked to see if the key matches.
    /// TODO(reece): Is optional required here? Should never fully loop the end since the table should never get full
    fn find_slot(&self, key: &K) -> Option<usize> {
        let hash = self.hash(key);
        let mut index = hash as usize % self.size;

        // If a slot hasn't been used yet, then the key cannot be further therefore it isn't in the map
        while self.table[index].has_been_used && self.table[index].key != *key && self.table[index].is_alive == true {
            index = (index + 1) % self.size;
        }

        return Some(index);
    }

    fn resize(&mut self) {
        let new_size = self.size * 2;

        let new_table = vec![HashTableEntry { key: Default::default(), value: Default::default(), is_alive: false, has_been_used: false }; new_size];
        let mut old_table = mem::replace(&mut self.table, new_table);
        let mut hash_count = 0;
        self.size = new_size;
        self.current_size = 0;

        let mut index = 0;
        let mut len = old_table.len();
        while index < len {
            if old_table[index].is_alive {
                let moved = old_table.remove(index);
                self.put(moved.key, moved.value).unwrap();
                len = old_table.len();
                hash_count += 1;
            } else {
                index += 1;
            }
        }
        self.current_size = hash_count;
    }

    fn put(&mut self, key: K, val: V) -> Result<(), &str> {
        let index = self.find_slot(&key).unwrap();
        self.table[index] = HashTableEntry {
            key: key.clone(),
            value: val,
            is_alive: true,
            has_been_used: true,
        };
        self.current_size += 1;
        if self.current_size >= self.size / 2 {
            self.resize();
        }
        return Ok(());
    }

    fn delete(&mut self, key: &K) {
        let index = self.find_slot(key).unwrap();
        if self.table[index].key == *key {
            self.table[index].is_alive = false;
        }
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return hasher.finish();
    }
}

fn main() {
    let mut hash_map = HashTable::new(2);

    hash_map.put(1, 1).unwrap();
    println!("{}", hash_map.get(&1).unwrap());
    hash_map.put(1, 11).unwrap();
    println!("{}", hash_map.get(&1).unwrap());
    hash_map.delete(&1);
    hash_map.put(1, 11).unwrap();
    println!("{}", hash_map.get(&1).unwrap());
    hash_map.put(3, 3).unwrap();
    println!("{}", hash_map.get(&3).unwrap());
    hash_map.put(5, 5).unwrap();
    println!("{}", hash_map.get(&5).unwrap());
    hash_map.put(-1, -1).unwrap();
    println!("{}", hash_map.get(&-1).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::HashTable;

    #[test]
    fn test_get_no_keys() {
        let hash_table: HashTable<String, i32> = HashTable::new(10);
        let result = hash_table.get(&"Key".to_string());
        assert_eq!(result, None);
    }

    #[test]
    fn test_put_and_get() {
        let mut hash_table = HashTable::new(10);
        hash_table.put("Test".to_string(), 3).unwrap();
        let result = hash_table.get(&"Test".to_string());
        assert_eq!(result, Some(&3));
    }

    #[test]
    fn test_put_and_get_multiple_keys() {
        let mut hash_table = HashTable::new(10);
        let result = hash_table.put("Key".to_string(), 1);
        assert!(result.is_ok());

        let result = hash_table.get(&"Key".to_string());
        assert_eq!(result, Some(&1));

        let result = hash_table.put("Test".to_string(), 3);
        assert!(result.is_ok());
        let result = hash_table.get(&"Test".to_string());
        assert_eq!(result, Some(&3));

        let result = hash_table.put("Test2".to_string(), 5);
        assert!(result.is_ok());
        let result = hash_table.get(&"Test2".to_string());
        assert_eq!(result, Some(&5));
    }

    #[test]
    fn test_put() {
        let mut hash_table = HashTable::new(10);
        let result = hash_table.put("Test".to_string(), 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_puts_different_keys() {
        let mut hash_table = HashTable::new(4);
        let result = hash_table.put("Test".to_string(), 10);
        assert!(result.is_ok());
        let result = hash_table.put("Test2".to_string(), 10);
        assert!(result.is_ok());
        let result = hash_table.put("Test3".to_string(), 10);
        assert!(result.is_ok());
        let result = hash_table.put("Test4".to_string(), 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete() {
        let mut hash_table: HashTable<String, i32> = HashTable::new(10);
        let result = hash_table.put("Test".to_string(), 10);
        assert!(result.is_ok());
        hash_table.delete(&"Test".to_string());
        let result = hash_table.get(&"Test".to_string());
        assert_eq!(result, None);
    }

    #[test]
    fn test_resize() {
        let mut hash_table: HashTable<String, i32> = HashTable::new(2);
        assert_eq!(hash_table.size, 2);
        let result = hash_table.put("Test".to_string(), 10);
        assert!(result.is_ok());
        assert_eq!(hash_table.size, 4);
        assert_eq!(hash_table.current_size, 1);
        let result = hash_table.get(&"Test".to_string());
        assert_eq!(result, Some(&10));
    }
}
