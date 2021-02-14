use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::collections::HashMap;
use rand::{self, seq::SliceRandom};
use crate::package::Package as Package;

pub fn create_test_set(num_packages: usize, max_weight: usize, max_price: usize, test_num: Option<u16>) {
    //! Creates a test set based off of the NAPCS list, randomizing prices and weights of objects
    //!
    //! If multiple boxes with the same good are generated, weight and price will be the same as the one
    //! that was generated earlier.
    //!
    //! Attributes:
    //!
    //! * num_packages: the number of packages to generate
    //! * max_weight: the upper limit of the randomly generated weight [0,max_weight)
    //! * max_price: the upper limit of the randomly generated price [0, max_price)
    //! * test_num: a number you can append to the test output.  Useful if you want to create multiple tests at once

    let output_file = match test_num {
        Some(t) => format!("./available_packages_{}.csv", t),
        None => format!("./available_packages.csv")
    };

    let item_names = read_input();

    let mut generated_items: HashMap<String, Package> = HashMap::new();

    for _ in 0..num_packages {
        let name = item_names.choose(&mut rand::thread_rng()).unwrap().clone();

        // PUT THE ITEMS INTO BOXES
        let weight = rand::thread_rng().gen_range(0..max_weight) as u16;
        let price = rand::thread_rng().gen_range(0..max_price) as u16;

        let temp: Package = Package::new(name, weight, price);
    }
}

fn read_input() -> Vec<String> {
    //! Reads in all of the possible item names from the NAPCS list and outputs them as a vector

    let input_file = File::open("F:\\git\\KnapsackAnalysis_Rust\\src\\NAPCS.txt").unwrap();
    let input_file = BufReader::new(input_file);

    let mut item_names = vec![];

    for (_, line) in input_file.lines().enumerate() {
        item_names.push(line.unwrap())
    }

    return item_names
}