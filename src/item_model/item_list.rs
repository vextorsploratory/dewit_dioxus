use super::list_item::ListItem;
use super::shared_item::SharedItem;
use std::collections::HashSet;
use maplit::hashset;

#[derive(Debug)]
pub struct ItemList {
    children: HashSet<SharedItem>,
}

impl ItemList {
    pub fn new() -> Self {
	Self{
	    children: hashset![],
	}
    }

    pub fn is_empty(&self) -> bool {
	return self.children.is_empty();
    }

    pub fn len(&self) -> usize {
	return self.children.len();
    }

    pub fn add(&mut self, child: SharedItem) {
	self.children.insert(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_list_new_is_empty() {
	assert!(ItemList::new().is_empty());
    }

    #[test]
    fn item_list_new_len_is_zero() {
	assert_eq!(0, ItemList::new().len());
    }

    #[test]
    fn add_to_new_makes_not_empty() {
	let item = SharedItem::new(ListItem::from_label("::_::"));
	
	let mut list = ItemList::new();
	list.add(item);

	assert!( !list.is_empty());
    }

    #[test]
    fn add_to_new_makes_len_one() {
	let mut list = ItemList::new();
	list.add(SharedItem::new(ListItem::from_label("::_::")));

	assert_eq!( 1, list.len());
    }


    #[test]
    fn add_twice_new_makes_len_two() {
	let mut list = ItemList::new();
	list.add(SharedItem::new(ListItem::from_label("::_::")));
	list.add(SharedItem::new(ListItem::from_label("::_::")));

	assert_eq!( 2, list.len());
    }

    #[test]
    fn add_same_twice_new_makes_len_one() {
	let mut list = ItemList::new();
	list.add(SharedItem::new(ListItem{id: "::SAME ID::".into(), label: "::ANY LABEL::".into(), children: ItemList::new()}));
	list.add(SharedItem::new(ListItem{id: "::SAME ID::".into(), label: "::COULD BE ANYTHING ELSE::".into(), children: ItemList::new()}));

	assert_eq!( 1, list.len());
    }

    
}
