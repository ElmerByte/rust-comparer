### A simple library to track changes in a HashMap over several iterations.

This library is designed to detect if a HashMap has been updated since it was last checked or if it is unchanged.
You can compare your hashmap with it's previous version to see if anything has been changed or added.

### Use cases
Simple use case when you need to return bool from a comparison:
 ```rust
use std::collections::HashMap;
use comparer::HashMapComparer;

let mut my_hashmap = HashMap::<u8, &str>::new();
let comparer = HashMapComparer::<u8, &str>::new();

my_hashmap.insert(1, "foo");
// HashMap has new values
assert_eq!(false, comparer.is_same_update(&my_hashmap));
// Hashmap has not received new values
assert_eq!(true, comparer.is_same_update(&my_hashmap));

my_hashmap.insert(2, "bar");
// HashMap has new values
assert_eq!(false, comparer.is_same_update(&my_hashmap));
```

Use case when you need to return values that have changed as a result of a comparison:
```rust
//Example usage
use std::collections::HashMap;
use comparer::HashMapComparer;

let comparer = HashMapComparer::<u8, &str>::new();
let mut my_hashmap = HashMap::<u8, &str>::new();
let mut results: Vec<HashMap<u8, &str>> = vec![];

my_hashmap.insert(1, "foo");
my_hashmap.insert(2, "bar");
my_hashmap.insert(4, "foo");

for i in 0..5 {
    my_hashmap.insert(i, "foo");
    results.push(comparer.update_and_compare(&my_hashmap).unwrap());
 }

assert_eq!(
    vec![
         // In the first comparison comparer always returns the whole hashmap because all values in it is new
         HashMap::<u8, &str>::from_iter(vec![(0, "foo"), (4, "foo"), (2, "bar"), (1, "foo")]),
         // Returns empty hashmap because value 1: "foo" didn't change
         HashMap::<u8, &str>::new(),
         // Returns hashmap with 2: "foo" because it was changed from 2: "bar"
         HashMap::<u8, &str>::from_iter(vec![(2, "foo")]),
         // Returns hashmap with 3: "foo" because it's a new value
         HashMap::<u8, &str>::from_iter(vec![(3, "foo")]),
         // Returns empty hashmap because value 4: "foo" didn't change
         HashMap::<u8, &str>::new(),
       ],
     results
 );
```
    
This library does not use any third-party crates, only crates from the standard rust library :)
