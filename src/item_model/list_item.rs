use super::id::Id;

pub struct ListItem {
    id: Id,
    label: String
}

impl ListItem {
    pub fn from_label(label: String) -> Self {
	Self {
	    id: Id::random(),
	    label: label,
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::vector_asserts::*;
    
    #[test]
    fn list_item_has_id() {
	let item_id = Id{id: "fred".to_string()};
	let item = ListItem{id: item_id.clone(), label: "".to_string()};

	assert_eq!(item_id, item.id);
    }

    #[test]
    fn list_item_has_label() {
	let item_label = "::A LABEL::".to_string();
	let item = ListItem{id: Id::random(), label: item_label.to_string()};

	assert_eq!(item_label, item.label);
    }

    #[test]
    fn list_item_has_constructor_with_only_label() {
	let item_label = "::ANY OLD LABEL::".to_string();

	let item = ListItem::from_label(item_label.clone());

	assert_eq!(item_label, item.label);
    }

    #[test]
    fn list_item_from_label_has_long_id() {
	let item_label = "::ANY OLD LABEL::".to_string();

	let item = ListItem::from_label(item_label.clone());

	assert!(32 <= item.id.id.len());
    }

    #[test]
    fn list_item_from_label_has_random_ids() {
	let ids: Vec<Id> = (0..1000)
	    .map (|_| ListItem::from_label("::_::".to_string()))
	    .map (|item| item.id.clone())
	    .collect();

	assert_no_duplicates(&ids);
    }
}
