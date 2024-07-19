A library to track changes in a HashMap over several iterations.

     //Example usage
     
     use std::collections::HashMap;
     use comparer::HashMapComparer;
     let comparer = HashMapComparer::<u8, &str>::new();
     let mut my_hashmap = HashMap::<u8, &str>::new();

     my_hashmap.insert(1, "foo");
     my_hashmap.insert(2, "bar");
     my_hashmap.insert(4, "foo");

     let mut results: Vec<HashMap<u8, &str>> = vec![];

     for i in 0..5 {
         my_hashmap.insert(i, "foo");
         results.push(comparer.update_and_compare(&my_hashmap).unwrap());
      }

    assert_eq!(
         vec![
              // In a first comparison comparer always returns whole hashmap because all values in it is new
              HashMap::<u8, &str>::from_iter(vec![(0, "foo"), (4, "foo"), (2, "bar"), (1, "foo")]),
              // Returns empty hashmap because value 1: "foo" didn't change
              HashMap::<u8, &str>::new(),
              // Returns hashmap with 2: "foo" because it was changed from 2: "bar"
              HashMap::<u8, &str>::from_iter(vec![(2, "foo")]),
              // Returns hashmap with 3: "foo" because it's a new value
              HashMap::<u8, &str>::from_iter(vec![(3, "foo")]),
              //Returns empty hashmap because value 4: "foo" didn't change
              HashMap::<u8, &str>::new(),
            ],
          results
      );



