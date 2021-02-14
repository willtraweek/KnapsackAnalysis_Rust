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
