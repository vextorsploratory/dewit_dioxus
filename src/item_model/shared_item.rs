use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug,Formatter,Result};

use super::list_item::ListItem;

#[derive(Clone)]
pub struct SharedItem(pub Rc<RefCell<ListItem>>);

impl SharedItem {
    pub fn new(item: ListItem) -> Self{
	SharedItem(Rc::new(RefCell::new(item)))
    }

    fn borrow(&self) -> std::cell::Ref<'_, ListItem> {
        self.0.borrow()
    }

    fn borrow_mut(&self) -> std::cell::RefMut<'_, ListItem> {
        self.0.borrow_mut()
    }    
}

impl PartialEq for SharedItem {
    fn eq(&self, other: &Self) -> bool {
	*self.borrow() == *other.borrow()
    }
}

impl Eq for SharedItem {}

impl Hash for SharedItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.borrow()).hash(state);
    }
}

impl Debug for SharedItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let item = self.0.borrow();
        write!(f, "{:?}", *item)
    }    
}
