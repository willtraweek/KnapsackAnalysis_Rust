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

        output
    }

}