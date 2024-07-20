use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::PoisonError;

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
    /// # Examples
    /// ```
    ///   use std::collections::HashMap;
    ///   use comparer::HashMapComparer;
    ///
    ///   let mut my_hashmap = HashMap::<u8, &str>::new();
    ///   let comparer = HashMapComparer::<u8, &str>::new();
    ///   my_hashmap.insert(1, "foo");
    ///   // HashMap has new values
    ///   assert_eq!(false, comparer.is_same_update(&my_hashmap));
    ///   // Hashmap has not recived new values
    ///   assert_eq!(true, comparer.is_same_update(&my_hashmap));
    ///```
    ///
    pub fn is_same_update(&self, new_map: &HashMap<K, V>) -> bool {
        let is_same = self.is_same(new_map);
        self.update(new_map);
        is_same
    }
    /// Updates last hashmap, compares new one to the last one and returns changed values.
    /// If you want to compare hashmap without updating last hashmap use``` compare()```.
    /// If you want to update hashmap without comparing it use ```update()``` and if you want to return bool instead of changed values use ```is_same()``` or ```is_same_update()```.
    ///
    /// # Examples
    /// ```
    ///   use std::collections::HashMap;
    ///   use comparer::HashMapComparer;
    ///   let comparer = HashMapComparer::<u8, &str>::new();
    ///   let mut my_hashmap = HashMap::<u8, &str>::new();

    ///   my_hashmap.insert(1, "foo");
    ///   my_hashmap.insert(2, "bar");
    ///   my_hashmap.insert(4, "foo");

    ///   let mut results: Vec<HashMap<u8, &str>> = vec![];

    ///   for i in 0..5 {
    ///       my_hashmap.insert(i, "foo");
    ///       results.push(comparer.update_and_compare(&my_hashmap).unwrap());
    ///   }

    ///   assert_eq!(
    ///       vec![
    ///           // In a first comparison comparer always returns whole hashmap because all values in it is new
    ///           HashMap::<u8, &str>::from_iter(vec![(0, "foo"), (4, "foo"), (2, "bar"), (1, "foo")]),
    ///           // Returns empty hashmap because value 1: "foo" didn't change
    ///           HashMap::<u8, &str>::new(),
    ///           // Returns hashmap with 2: "foo" because it was changed from 2: "bar"
    ///           HashMap::<u8, &str>::from_iter(vec![(2, "foo")]),
    ///           // Returns hashmap with 3: "foo" because it's a new value
    ///           HashMap::<u8, &str>::from_iter(vec![(3, "foo")]),
    ///            //Returns empty hashmap because value 4: "foo" didn't change
    ///           HashMap::<u8, &str>::new(),
    ///       ],
    ///       results
    ///   );
    /// ```

    pub fn update_and_compare(
        &self,
        new_map: &HashMap<K, V>,
    ) -> Result<HashMap<K, V>, PoisonError<K>> {
        let mut last_map = self.last_map.lock().unwrap();
        let mut changed_values: HashMap<K, V> = HashMap::new();
        if !last_map.is_empty() {
            for (key, value) in new_map.iter() {
                if last_map.contains_key(key) {
                    if value != last_map.get(key).unwrap() {
                        changed_values.insert(key.clone(), value.clone());
                    }
                } else {
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

    pub fn compare(&self, new_map: &HashMap<K, V>) -> Result<HashMap<K, V>, PoisonError<K>> {
        let last_map = self.last_map.lock().unwrap();
        let mut changed_values: HashMap<K, V> = HashMap::new();
        if !last_map.is_empty() {
            for (key, value) in new_map.iter() {
                if last_map.contains_key(key) {
                    if value != last_map.get(key).unwrap() {
                        changed_values.insert(key.clone(), value.clone());
                    }
                } else {
                    changed_values.insert(key.clone(), value.clone());
                }
            }
            return Ok(changed_values);
        }
        Ok(new_map.clone())
    }
}

/// HashMapC
#[derive(Debug, Clone)]
pub struct HashMapComparer<K: Clone + Eq + Hash, V: Clone + PartialEq> {
    last_map: Arc<Mutex<HashMap<K, V>>>,
}
impl<K: Clone + Eq + Hash, V: Clone + PartialEq> Default for HashMapComparer<K, V> {
    fn default() -> Self {
        HashMapComparer::new()
    }
}
