use dioxus::prelude::*;
use super::list_item::ListItem;

#[derive(Clone, PartialEq, Props)]
pub struct RootList {
    pub lists: Signal<Vec<Signal<ListItem>>>,
}

impl RootList {
    pub fn new() -> Self {
	Self {
	    lists: use_signal(|| vec![]),
	}
    }

    pub fn is_empty(&self) -> bool {
	self.lists.read().is_empty()
    }

    pub fn add(&mut self, item: ListItem) {
	let new_list = use_signal(|| item);
	self.lists.write().push(new_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::dioxus::init_dioxus_runtime;
    
    #[test]
    fn new_makes_empty_rootlist() {
	let _gaurd = init_dioxus_runtime;
	
	let roots = RootList::new();

	assert!(roots.is_empty());
    }

    #[test]
    fn add_makes_not_empty_rootlist() {
	let _gaurd = init_dioxus_runtime;

	let mut roots = RootList::new();
	roots.add(ListItem::from_label("::an_item::"));

	assert!( !roots.is_empty());
    }
    
}
