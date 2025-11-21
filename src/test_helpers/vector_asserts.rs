use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub fn assert_no_duplicates<T: Eq + Hash + Debug>(v: &Vec<T>) {
    let mut test_set = HashSet::new();
    for x in v {
	assert!(test_set.insert(x),"duplicate found in ids: {:?}", x);
    }
}
