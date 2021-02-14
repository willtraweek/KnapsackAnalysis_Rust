use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::collections::{hash_set, HashSet};
use rand::{self, seq::SliceRandom};

pub fn create_test_set(num_boxes: usize, max_weight: usize, max_price: usize, test_num: Option<u16>) {
    //! Creates a test set based off of the NAPCS list, randomizing prices and weights of objects
    //!
    //! If multiple boxes with the same good are generated, weight and price will be the same as the one
    //! that was generated earlier.
    //!
    //! Attributes:
    //!
    //! * num_boxes: the number of boxes to generate
    //! * max_weight: the upper limit of the randomly generated weight [0,max_weight)
    //! * max_price: the upper limit of the randomly generated price [0, max_price)
    //! * test_num: a number you can append to the test output.  Useful if you want to create multiple tests at once

    let output_file = match test_num {
        Some(t) => format!("./available_boxes_{}.csv", t),
        None => format!("./available_boxes.csv")
    };

    let item_names = read_input();

    let mut generated_items: HashSet<String> = HashSet::new();

    for i in 0..num_boxes {
        let item_name = item_names.choose(&mut rand::thread_rng());

        // PUT THE ITEMS INTO BOXES
    }
}

fn read_input() -> Vec<String> {
    //! Reads in all of the possible item names from the NAPCS list and outputs them as a vector

    let input_file = File::open("F:\\git\\KnapsackAnalysis_Rust\\src\\NAPCS.txt").unwrap();
    let input_file = BufReader::new(input_file);

    let mut item_names = vec![];

    for (i, line) in input_file.lines().enumerate() {
        item_names.push(line.unwrap())
    }

    return item_names
}