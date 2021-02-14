use crate::package::Package as Package;
use std::collections::HashMap;
use std::cmp::{Ordering, Eq, PartialOrd, PartialEq};

#[derive(Eq)]
pub struct Truck {
    /// Truck to be filled with packages
    ///
    /// Attributes:
    /// * capacity: the maximum weight a truck can hold
    /// * weight: the current weight
    /// * value: the value of all of the boxes in the truck
    /// * boxes: A map of all of the boxes that are in the truck
    capacity: u32,
    weight: u32,
    value: u32,
    boxes: HashMap<String, u16>
}


impl Ord for Truck {
    fn cmp (&self, other: &Self) -> Ordering {
        if self.capacity < other.capacity {
            return Ordering::Less;
        } else if self.weight < other.weight {
            return Ordering::Less;
        } else if self.value < other.value {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl PartialOrd for Truck {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Truck {
    fn eq(&self, other: &Self) -> bool {
        return (self.capacity == other.capacity) && (self.weight == other.weight) && (self.value == other.value)
    }
}