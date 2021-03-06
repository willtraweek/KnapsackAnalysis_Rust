use crate::package::Package;
use std::collections::HashMap;
use std::cmp::{Ordering, Eq, PartialOrd, PartialEq};

#[derive(Eq)]
pub struct Truck {
    /// Truck to be filled with packages
    ///
    /// Attributes:
    /// * capacity: the maximum weight a truck can hold
    /// * weight: the current weight
    /// * value: the value of all of the packages in the truck
    /// * packages: A map of all of the packages that are in the truck
    capacity: usize,
    weight: usize,
    value: usize,
    pub packages: HashMap<Package, u16>
}

impl Truck {
    pub fn new(capacity: usize) -> Truck {
        Truck {
            capacity,
            weight: 0,
            value: 0,
            packages: HashMap::new()
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn value(&self) -> usize { self.value }

    pub fn leftover_space(&self) -> usize{
        self.capacity - self.weight
    }

    pub fn add_package(&mut self, package: &Package) -> bool {
        //! Tries to add a package to the truck.  If there's not enough space, it returns an error, else
        //! it returns None
        if package.weight() <= self.leftover_space() {
            if self.packages.contains_key(package){
                * self.packages.get_mut(package).unwrap() += 1;
            } else {
                self.packages.insert(package.clone(), 1);
            }

            self.weight += package.weight();
            self.value += package.price();
            true
        } else {
            // THIS PACKAGE IS TOO HEAVY TO FIT, SO WE CAN'T ADD IT
            false
        }
    }

    pub fn remove_package(&mut self, package: &Package) -> bool {
        if self.packages.contains_key(package) {
            * self.packages.get_mut(package).unwrap() -= 1;
            let num_packages = * self.packages.get(package).unwrap();

            self.weight -= package.weight();
            self.value -= package.price();

            // IF THE REMOVED ITEM WAS THE FINAL ONE OF ITS TYPE, REMOVE IT
            if num_packages == 0 {
                self.packages.remove(package);
            }

            true
        } else {
            // THERE ARE NO PACKAGES OF THIS TYPE IN THE TRUCK, SO WE CAN'T REMOVE IT
            false
        }
    }

    pub fn clone(& self) -> Truck {
        let mut output = Truck {
            capacity: self.capacity,
            weight: self.weight,
            value: self.value,
            packages: HashMap::new()
        };

        output.packages = self.packages.clone();
        output
    }
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