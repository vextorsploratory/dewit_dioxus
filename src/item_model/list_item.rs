use super::id::Id;
use super::item_list::ItemList;

use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct ListItem {
    pub id: Id,
    pub label: String,
    pub children: ItemList,
	
}

impl ListItem {
    pub fn from_label(label: &str) -> Self {
	Self {
	    id: Id::random(),
	    label: label.to_string(),
	    children: ItemList::new(),
	}
    }
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
	self.id == other.id
    }
}

impl Eq for ListItem {}

impl Hash for ListItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::vector_asserts::*;
    
    #[test]
    fn list_item_has_id() {
	let item_id = Id{id: "fred".to_string()};
	let item = ListItem{id: item_id.clone(), label: "".to_string(), children: ItemList::new()};

	assert_eq!(item_id, item.id);
    }

    #[test]
    fn list_item_has_label() {
	let item_label = "::A LABEL::".to_string();
	let item = ListItem{id: Id::random(), label: item_label.to_string(), children: ItemList::new()};

	assert_eq!(item_label, item.label);
    }

    #[test]
    fn list_item_has_constructor_with_only_label() {
	let item_label = "::ANY OLD LABEL::";

	let item = ListItem::from_label(item_label);

	assert_eq!(item_label, item.label);
    }

    #[test]
    fn list_item_from_label_has_long_id() {
	let item_label = "::ANY OLD LABEL::";

	let item = ListItem::from_label(item_label);

	assert!(32 <= item.id.id.len());
    }

    #[test]
    fn list_item_from_label_has_random_ids() {
	let ids: Vec<Id> = (0..1000)
	    .map (|_| ListItem::from_label("::_::"))
	    .map (|item| item.id.clone())
	    .collect();

	assert_no_duplicates(&ids);
    }

    #[test]
    fn two_items_equal_if_same_id() {
	let item1 = ListItem{id: Id::random(), label: "::_::".to_string(), children: ItemList::new()};
	let item2 = ListItem{id: item1.id.clone(), label: "Som other id".to_string(), children: ItemList::new()};

	assert_eq!(item1, item2);
    }

    #[test]
    fn two_items_not_equal_if_diff_id() {
	let item1 = ListItem{id: Id::random(), label: "::ANY LABEL::".to_string(), children: ItemList::new()};
	let item2 = ListItem{id: Id::random(), label: "::A DIFFERENT LABEL::".to_string(), children: ItemList::new()};

	assert_ne!(item1, item2);
    }

    #[test]
    fn hash_equal_to_id_hash() {
	use std::collections::HashSet;

	let mut set = HashSet::new();

	set.insert(ListItem { id: "1".into(), label: "A".into(), children: ItemList::new()});
	set.insert(ListItem { id: "1".into(), label: "B".into(), children: ItemList::new()});
    }
    
}
