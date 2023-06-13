use serde::{Deserialize, Serialize};
use std::rc::Weak;
use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub obtained: bool
}

impl Item {
    pub fn new(name: String) -> Self {
        Self { name: name, obtained: false }
    }

    pub fn toggle_obtained(&mut self) {
        self.obtained = !self.obtained;
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemRequirement {
    pub items: Vec<Weak<RefCell<Item>>>,
    pub completed: bool
}

impl ItemRequirement {
    pub fn new() -> Self {
        Self {items: Vec::new(), completed: false}
    }

    pub fn add_item(&mut self, item: Weak<RefCell<Item>>) {
        self.items.push(item);
    }

    fn clean_items(&mut self) {
        self.items.retain(|item| {
            match item.upgrade() {
                None => false,
                Some(_) => true
            }
        })
    }

    pub fn check_completed(&mut self) {
        self.clean_items();
        for item_ref in self.items.iter() {
            let obtained = item_ref.upgrade().unwrap().borrow().obtained;
            if !obtained {
                self.completed = false;
                return;
            }
        }
        self.completed = true;
    }
}