use crate::items::ItemRequirement;
use std::rc::Weak;
use std::cell::RefCell;

pub struct Check {
    pub name: String,
    pub item_requirements: Vec<ItemRequirement>
}

impl Check {
    pub fn new(name: String) -> Self {
        Self {name: name, item_requirements: Vec::new()}
    }

    pub fn add_item_requirement(&mut self, item_requirement: ItemRequirement) {
        self.item_requirements.push(item_requirement);
    }
}

pub struct Location {
    pub name: String,
    pub checks: Vec<Check>
}

impl Location {
    pub fn new(name: String) -> Self {
        Self {name: name, checks: Vec::new()}
    }

    pub fn add_check(&mut self, check: Check) {
        self.checks.push(check);
    }
}