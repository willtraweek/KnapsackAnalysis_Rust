use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

#[derive(Eq)]
#[derive(Clone)]
pub struct Package {
    name: String,
    weight: usize,
    price: usize
}

impl Package {
    pub fn new(name: String, weight: usize, price: usize) -> Package {
        Package {
            name,
            weight,
            price
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn weight(&self) -> usize {
        self.weight
    }

    pub fn price(&self) -> usize {
        self.price
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name
    }
}

impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}