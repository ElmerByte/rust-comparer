use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;

#[cfg(feature = "dashmap")]
use dashmap::DashMap;

/// HashMapComparer
/// struct that contains `last_map` and implements several methods for it
#[derive(Debug, Clone)]
pub struct HashMapComparer<K: Clone + Eq + Hash, V: Clone + PartialEq> {
    last_map: Arc<Mutex<HashMap<K, V>>>,
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> Default for HashMapComparer<K, V> {
    fn default() -> Self {
        HashMapComparer::new()
    }
}

impl<K: Clone + Eq + Hash, V: Clone + PartialEq> HashMapComparer<K, V> {
    pub fn new() -> Self {
        Self {
            last_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Clones last hashmap
    pub fn clone_last(&self) -> HashMap<K, V> {
        self.last_map.lock().unwrap().clone()
    }

    /// Checks if last hashmap is the same as new one
    pub fn is_same(&self, comparable: &HashMap<K, V>) -> bool {
        self.last_map.lock().unwrap().iter().eq(comparable)
    }

    /// Updates last hashmap to a new value
    pub fn update(&self, new_map: &HashMap<K, V>) {
        self.last_map.lock().unwrap().clone_from(new_map);
    }

    /// Checks if last hashmap is the same as new one and updates it to be that new value
    pub fn is_same_update(&self, new_map: &HashMap<K, V>) -> bool {
        let is_same = self.is_same(new_map);
        self.update(new_map);
        is_same
    }

    /// Updates last hashmap, compares new one to the last one and returns changed values.
    pub fn update_and_compare(
        &self,
        new_map: &HashMap<K, V>,
    ) -> Result<HashMap<K, V>, Box<dyn std::error::Error>> {
        let mut last_map = self.last_map.lock().unwrap();
        let mut changed_values: HashMap<K, V> = HashMap::new();
        if !last_map.is_empty() {
            for (key, value) in new_map {
                if !last_map.contains_key(key) || value != last_map.get(key).expect("None Value") {
                    changed_values.insert(key.clone(), value.clone());
                }
            }
            last_map.clone_from(new_map);
            return Ok(changed_values);
        }
        last_map.clone_from(new_map);
        Ok(new_map.clone())
    }

    /// Compares new hashmap to the last one and returns changed values
    pub fn compare(
        &self,
        new_map: &HashMap<K, V>,
    ) -> Result<HashMap<K, V>, Box<dyn std::error::Error>> {
        let last_map = self.last_map.lock().unwrap();
        let mut changed_values: HashMap<K, V> = HashMap::new();
        if !last_map.is_empty() {
            for (key, value) in new_map.iter() {
                if !last_map.contains_key(key) || value != last_map.get(key).unwrap() {
                    changed_values.insert(key.clone(), value.clone());
                }
            }
            return Ok(changed_values);
        }
        Ok(new_map.clone())
    }
}

#[cfg(feature = "dashmap")]
impl<K: Clone + Eq + Hash, V: Clone + PartialEq> HashMapComparer<K, V> {
    /// Checks if last hashmap is the same as DashMap
    pub fn is_same_dashmap(&self, comparable: &DashMap<K, V>) -> bool {
        let last_map = self.last_map.lock().unwrap();
        if last_map.len() != comparable.len() {
            return false;
        }
        last_map.iter().all(|(k, v)| {
            comparable.get(k).map_or(false, |entry| entry.value() == v)
        })
    }

    /// Updates last hashmap from DashMap
    pub fn update_dashmap(&self, new_map: &DashMap<K, V>) {
        let mut last_map = self.last_map.lock().unwrap();
        last_map.clear();
        for entry in new_map.iter() {
            last_map.insert(entry.key().clone(), entry.value().clone());
        }
    }

    /// Checks if last hashmap is the same as DashMap and updates it
    pub fn is_same_update_dashmap(&self, new_map: &DashMap<K, V>) -> bool {
        let is_same = self.is_same_dashmap(new_map);
        self.update_dashmap(new_map);
        is_same
    }

    /// Updates last hashmap from DashMap and returns changed values
    pub fn update_and_compare_dashmap(
        &self,
        new_map: &DashMap<K, V>,
    ) -> Result<HashMap<K, V>, Box<dyn std::error::Error>> {
        let mut last_map = self.last_map.lock().unwrap();
        let mut changed_values: HashMap<K, V> = HashMap::new();
        
        if !last_map.is_empty() {
            for entry in new_map.iter() {
                let key = entry.key();
                let value = entry.value();
                if !last_map.contains_key(key) || value != last_map.get(key).unwrap() {
                    changed_values.insert(key.clone(), value.clone());
                }
            }
            last_map.clear();
            for entry in new_map.iter() {
                last_map.insert(entry.key().clone(), entry.value().clone());
            }
            return Ok(changed_values);
        }
        
        for entry in new_map.iter() {
            last_map.insert(entry.key().clone(), entry.value().clone());
        }
        
        Ok(last_map.clone())
    }

    /// Compares DashMap to last hashmap and returns changed values
    pub fn compare_dashmap(
        &self,
        new_map: &DashMap<K, V>,
    ) -> Result<HashMap<K, V>, Box<dyn std::error::Error>> {
        let last_map = self.last_map.lock().unwrap();
        let mut changed_values: HashMap<K, V> = HashMap::new();
        
        if !last_map.is_empty() {
            for entry in new_map.iter() {
                let key = entry.key();
                let value = entry.value();
                if !last_map.contains_key(key) || value != last_map.get(key).unwrap() {
                    changed_values.insert(key.clone(), value.clone());
                }
            }
            return Ok(changed_values);
        }
        
        for entry in new_map.iter() {
            changed_values.insert(entry.key().clone(), entry.value().clone());
        }
        Ok(changed_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Helper to create a populated map for testing
    fn sample_map() -> HashMap<u8, &'static str> {
        let mut map = HashMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(4, "four");
        map
    }

    #[test]
    fn test_new_and_default() {
        let comparer1 = HashMapComparer::<u32, String>::new();
        let comparer2 = HashMapComparer::<u32, String>::default();

        assert!(comparer1.clone_last().is_empty());
        assert!(comparer2.clone_last().is_empty());
    }

    #[test]
    fn test_is_same_and_is_same_update() {
        let comparer = HashMapComparer::<u8, &str>::new();

        let mut map = HashMap::new();
        assert!(comparer.clone_last().is_empty());

        // First insert → different
        map.insert(1, "one");
        assert!(!comparer.is_same(&map));
        assert!(!comparer.is_same_update(&map)); // false + updates

        // Same map → should now be equal
        assert!(comparer.is_same(&map));
        assert!(comparer.is_same_update(&map)); // true + updates (no-op)

        // Change value
        map.insert(1, "uno");
        assert!(!comparer.is_same(&map));
        assert!(!comparer.is_same_update(&map));

        // Same again
        assert!(comparer.is_same(&map));
        assert!(comparer.is_same_update(&map));
    }

    #[test]
    fn test_compare_first_time_returns_full_map() {
        let comparer = HashMapComparer::<u8, &str>::new();

        let map = sample_map();
        let result = comparer.compare(&map).unwrap();

        assert_eq!(result, map);
        // Important: compare() does NOT update the internal state
        assert!(comparer.clone_last().is_empty());
    }

    #[test]
    fn test_update_and_compare_full_sequence() {
        let comparer = HashMapComparer::<u8, &str>::new();
        let mut map = HashMap::new();

        // Step 1: First update → returns full map
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(4, "four");

        let changed1 = comparer.update_and_compare(&map).unwrap();
        assert_eq!(changed1, map);

        // Step 2: Same map → empty changes
        let changed2 = comparer.update_and_compare(&map).unwrap();
        assert!(changed2.is_empty());

        // Step 3: Change existing value
        map.insert(2, "dos");
        let changed3 = comparer.update_and_compare(&map).unwrap();
        assert_eq!(
            changed3,
            HashMap::from_iter(vec![(2, "dos")])
        );

        // Step 4: Add new key
        map.insert(3, "three");
        let changed4 = comparer.update_and_compare(&map).unwrap();
        assert_eq!(
            changed4,
            HashMap::from_iter(vec![(3, "three")])
        );

        // Step 5: Change existing + add new
        map.insert(1, "uno");
        map.insert(5, "five");
        let changed5 = comparer.update_and_compare(&map).unwrap();
        assert_eq!(
            changed5,
            HashMap::from_iter(vec![(1, "uno"), (5, "five")])
        );

        // Step 6: No change
        let changed6 = comparer.update_and_compare(&map).unwrap();
        assert!(changed6.is_empty());
    }

    #[test]
    fn test_update_separately_then_compare() {
        let comparer = HashMapComparer::<String, i32>::new();

        let mut map = HashMap::new();
        map.insert("a".to_string(), 10);
        map.insert("b".to_string(), 20);

        comparer.update(&map);

        let mut map2 = map.clone();
        map2.insert("c".to_string(), 30);
        map2.insert("a".to_string(), 100);

        let changed = comparer.compare(&map2).unwrap();

        let mut expected = HashMap::new();
        expected.insert("c".to_string(), 30);
        expected.insert("a".to_string(), 100);

        assert_eq!(changed, expected);
    }

    #[test]
    fn test_clone_last_reflects_updates() {
        let comparer = HashMapComparer::<u32, bool>::new();

        let mut map = HashMap::new();
        map.insert(42, true);
        map.insert(100, false);

        comparer.update(&map);
        assert_eq!(comparer.clone_last(), map);

        map.insert(7, true);
        comparer.update(&map);
        assert_eq!(comparer.clone_last(), map);
    }

    #[test]
    fn test_different_value_type() {
        let comparer = HashMapComparer::<String, Vec<i32>>::new();

        let mut map = HashMap::new();
        map.insert("nums".to_string(), vec![1, 2, 3]);
        map.insert("more".to_string(), vec![10, 20]);

        let changed1 = comparer.update_and_compare(&map).unwrap();
        assert_eq!(changed1, map);

        // Change one value
        let mut map2 = map.clone();
        map2.insert("nums".to_string(), vec![1, 2, 4]);

        let changed2 = comparer.update_and_compare(&map2).unwrap();
        let mut expected = HashMap::new();
        expected.insert("nums".to_string(), vec![1, 2, 4]);

        assert_eq!(changed2, expected);
    }

    #[test]
    fn test_empty_map_handling() {
        let comparer = HashMapComparer::<u8, &str>::new();

        let empty = HashMap::new();
        assert!(comparer.is_same(&empty));
        assert!(comparer.is_same_update(&empty));

        let changed = comparer.update_and_compare(&empty).unwrap();
        assert!(changed.is_empty());

        let compared = comparer.compare(&empty).unwrap();
        assert!(compared.is_empty());
    }

    #[cfg(feature = "dashmap")]
    mod dashmap_tests {
        use super::*;
        use dashmap::DashMap;

        fn sample_dashmap() -> DashMap<u8, &'static str> {
            let map = DashMap::new();
            map.insert(1, "one");
            map.insert(2, "two");
            map.insert(4, "four");
            map
        }

        #[test]
        fn test_is_same_dashmap() {
            let comparer = HashMapComparer::<u8, &str>::new();
            let dashmap = DashMap::new();

            assert!(comparer.is_same_dashmap(&dashmap));

            dashmap.insert(1, "one");
            assert!(!comparer.is_same_dashmap(&dashmap));

            comparer.update_dashmap(&dashmap);
            assert!(comparer.is_same_dashmap(&dashmap));

            dashmap.insert(1, "uno");
            assert!(!comparer.is_same_dashmap(&dashmap));
        }

        #[test]
        fn test_update_dashmap() {
            let comparer = HashMapComparer::<u8, &str>::new();
            let dashmap = sample_dashmap();

            comparer.update_dashmap(&dashmap);
            
            let last = comparer.clone_last();
            assert_eq!(last.len(), 3);
            assert_eq!(last.get(&1), Some(&"one"));
            assert_eq!(last.get(&2), Some(&"two"));
            assert_eq!(last.get(&4), Some(&"four"));
        }

        #[test]
        fn test_is_same_update_dashmap() {
            let comparer = HashMapComparer::<u8, &str>::new();
            let dashmap = DashMap::new();

            dashmap.insert(1, "one");
            assert!(!comparer.is_same_update_dashmap(&dashmap));
            assert!(comparer.is_same_update_dashmap(&dashmap));

            dashmap.insert(1, "uno");
            assert!(!comparer.is_same_update_dashmap(&dashmap));
            assert!(comparer.is_same_update_dashmap(&dashmap));
        }

        #[test]
        fn test_compare_dashmap_first_time() {
            let comparer = HashMapComparer::<u8, &str>::new();
            let dashmap = sample_dashmap();

            let result = comparer.compare_dashmap(&dashmap).unwrap();

            assert_eq!(result.len(), 3);
            assert_eq!(result.get(&1), Some(&"one"));
            assert_eq!(result.get(&2), Some(&"two"));
            assert_eq!(result.get(&4), Some(&"four"));
            
            // compare_dashmap doesn't update
            assert!(comparer.clone_last().is_empty());
        }

        #[test]
        fn test_update_and_compare_dashmap_full_sequence() {
            let comparer = HashMapComparer::<u8, &str>::new();
            let dashmap = DashMap::new();

            // First update
            dashmap.insert(1, "one");
            dashmap.insert(2, "two");
            dashmap.insert(4, "four");

            let changed1 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert_eq!(changed1.len(), 3);

            // Same map
            let changed2 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert!(changed2.is_empty());

            // Change existing value
            dashmap.insert(2, "dos");
            let changed3 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert_eq!(changed3.len(), 1);
            assert_eq!(changed3.get(&2), Some(&"dos"));

            // Add new key
            dashmap.insert(3, "three");
            let changed4 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert_eq!(changed4.len(), 1);
            assert_eq!(changed4.get(&3), Some(&"three"));

            // Change existing + add new
            dashmap.insert(1, "uno");
            dashmap.insert(5, "five");
            let changed5 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert_eq!(changed5.len(), 2);
            assert_eq!(changed5.get(&1), Some(&"uno"));
            assert_eq!(changed5.get(&5), Some(&"five"));

            // No change
            let changed6 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert!(changed6.is_empty());
        }

        #[test]
        fn test_update_dashmap_separately_then_compare() {
            let comparer = HashMapComparer::<String, i32>::new();

            let dashmap = DashMap::new();
            dashmap.insert("a".to_string(), 10);
            dashmap.insert("b".to_string(), 20);

            comparer.update_dashmap(&dashmap);

            dashmap.insert("c".to_string(), 30);
            dashmap.insert("a".to_string(), 100);

            let changed = comparer.compare_dashmap(&dashmap).unwrap();

            assert_eq!(changed.len(), 2);
            assert_eq!(changed.get("c"), Some(&30));
            assert_eq!(changed.get("a"), Some(&100));
        }

        #[test]
        fn test_empty_dashmap_handling() {
            let comparer = HashMapComparer::<u8, &str>::new();
            let empty = DashMap::new();

            assert!(comparer.is_same_dashmap(&empty));
            assert!(comparer.is_same_update_dashmap(&empty));

            let changed = comparer.update_and_compare_dashmap(&empty).unwrap();
            assert!(changed.is_empty());

            let compared = comparer.compare_dashmap(&empty).unwrap();
            assert!(compared.is_empty());
        }

        #[test]
        fn test_dashmap_with_different_value_types() {
            let comparer = HashMapComparer::<String, Vec<i32>>::new();

            let dashmap = DashMap::new();
            dashmap.insert("nums".to_string(), vec![1, 2, 3]);
            dashmap.insert("more".to_string(), vec![10, 20]);

            let changed1 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert_eq!(changed1.len(), 2);

            dashmap.insert("nums".to_string(), vec![1, 2, 4]);

            let changed2 = comparer.update_and_compare_dashmap(&dashmap).unwrap();
            assert_eq!(changed2.len(), 1);
            assert_eq!(changed2.get("nums"), Some(&vec![1, 2, 4]));
        }
    }
}
