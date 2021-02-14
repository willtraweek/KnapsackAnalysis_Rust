use std::collections::{BinaryHeap, HashMap};
use crate::truck::Truck;
use crate::package::Package;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct Warehouse {
    trucks: BinaryHeap<Truck>,
    packages: HashMap<String, u16>,
    capacity: usize,
    value: usize
}

impl Warehouse {
    pub fn new(input_file_path: String) -> Warehouse{
        let output = Warehouse {
            trucks: BinaryHeap::new(),
            packages: HashMap::new(),
            capacity: 0,
            value: 0
        };

        output.import_boxes(input_file_path);

        output
    }

    fn import_boxes(&self, input_file_path: String) {
        let input_file = File::open(input_file_path).expect("Can't open the file for the warehouse");

        for line in BufReader::new(input_file).lines() {
            let line = line.split(",");
            name = line.1;

            // THIS PREVENTS THE FIRST LINE FROM BEING READ IN
            if name == "name" { continue };

            weight = line.2.parse::<usize>().unwrap();
            price = line.3.parse::<usize>().unwrap();

            self.add_package(Package::new(name, weight, price));
        }
    }

    pub fn add_truck(&mut self, truck: Truck) {
        self.trucks.push(truck);
        self.capacity += truck.capacity();
    fn add_package(&mut self, package: Package) {
        if self.packages.contains_key(package.name()) {
            * self.packages.get_mut(package.name()).unwrap() += 1;
        } else {
            self.packages.insert(package.name().clone(), 1);
        }

        self.value += package.price();
    }
}