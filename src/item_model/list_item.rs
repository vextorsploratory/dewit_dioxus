pub struct ListItem {
    id: String,
    label: String
}

impl ListItem {
    pub fn from_label(label: String) -> Self {
	Self {
	    id: "dummy".to_string(),
	    label: label,
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_item_has_id() {
	let item_id = "fred".to_string();
	let item = ListItem{id: item_id.clone(), label: "".to_string()};

	assert_eq!(item_id, item.id);
    }

    #[test]
    fn list_item_has_label() {
	let item_label = "::A LABEL::".to_string();
	let item = ListItem{id: "".to_string(), label: item_label.to_string()};

	assert_eq!(item_label, item.label);
    }

    #[test]
    fn list_item_has_constructor_with_only_label() {
	let item_label = "::ANY OLD LABEL::".to_string();

	let item = ListItem::from_label(item_label.clone());

	assert_eq!(item_label, item.label);
    }
}
