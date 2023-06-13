mod items;
mod locations;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use items::{Item, ItemRequirement};
use locations::Check;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut items: Vec<Rc<RefCell<Item>>> = Vec::new();
        let item: Rc<RefCell<Item>> = Rc::new(RefCell::new(Item::new("Prueba".to_string())));
        items.push(item);
        let mut item_requirement = ItemRequirement::new();
        item_requirement.add_item(Rc::downgrade(&items[0]));
        item_requirement.add_item(Weak::new());
        items[0].borrow_mut().toggle_obtained();
        item_requirement.check_completed();
        assert_eq!(item_requirement.completed, true)
    }
}
