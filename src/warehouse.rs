use std::collections::{BinaryHeap, HashMap};
use crate::truck::Truck;
use crate::package::Package;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct Warehouse {
    trucks: BinaryHeap<Truck>,
    packages: HashMap<Package, u16>,
    capacity: usize,
    value: usize
}

impl Warehouse {
    pub fn new(input_file_path: String) -> Warehouse{
        let mut output = Warehouse {
            trucks: BinaryHeap::new(),
            packages: HashMap::new(),
            capacity: 0,
            value: 0
        };

        output.import_boxes(input_file_path);

        output
    }

    fn import_packages(&mut self, input_file_path: String) {
        let input_file = File::open(input_file_path).expect("Can't open the file for the warehouse");

        for line in BufReader::new(input_file).lines() {
            let line = line.unwrap();
            let line = line.split(",").collect::<Vec<&str>>();
            let name = (* line.get(0).unwrap()).to_string();

            // THIS PREVENTS THE FIRST LINE FROM BEING READ IN
            if name == "name" { continue };

            let weight = (* line.get(1).unwrap()).parse::<usize>().unwrap();
            let price = (* line.get(2).unwrap()).parse::<usize>().unwrap();

            self.add_package(&Package::new(name, weight, price));
        }
    }

    pub fn add_truck(&mut self, truck: Truck) {
        self.capacity += truck.capacity();
        self.trucks.push(truck);
    }

    fn add_package(&mut self, package: &Package) {
        if self.packages.contains_key(package) {
            * self.packages.get_mut(package).unwrap() += 1;
        } else {
            self.packages.insert(package.clone(), 1);
        }

        self.value += package.price();
    }

    fn remove_package(&mut self, package: &Package) {
        if ! self.packages.contains_key(package){
            panic!("Cannot remove a package that doesn't exist from the warehouse")
        }

        // REMOVE THIS PACKAGE'S VALUE AND DECREMENT THE COUNT BY 1
        self.value -= package.price();
        * self.packages.get_mut(package).unwrap() -= 1;

        // IF THERE ARE NO MORE PACKAGES OF THIS TYPE, REMOVE THAT LISTING FROM THE PACKAGES LIST
        if (* self.packages.get(package).unwrap()) == 0 {
            self.packages.remove(package);
        }
    }

    fn load_package(&mut self, package: &Package, mut truck: Truck) {
        let temp = truck.add_package(&package);

        if temp {
            self.remove_package(package)
        }
    }

    fn unload_package(&mut self, package: &Package, mut truck: Truck) {
        let temp = truck.remove_package(&package);

        if temp {
            self.add_package(package);
        }
    }

    pub fn num_packages(&self) -> usize {
        self.packages.len()
    }
