use std::fmt::{Debug,Display,Formatter,Result};
use std::hash::Hash;
use uuid::Uuid;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Id {
    id: String,
}

impl Id {
    fn random() -> Self {
	Self{
	    id: Uuid::new_v4().urn().to_string(),
	}
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	write!(f, "{}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use crate::test_helpers::vector_asserts::*;
    
    #[test]
    fn id_from_random_is_never_same() {
	let ids: Vec<Id> = (0..1000)
	    .map (|_| Id::random())
	    .collect();

	assert_no_duplicates(&ids);
    }

    #[test]
    fn id_from_random_is_at_least_32_chars() {
	let id = Id::random().id;

	assert!(32 <= id.len());
    }

    #[test]
    fn id_to_string_is_id_field() {
	let id = Id::random();

	assert_eq!(id.id, id.to_string());
    }
}
