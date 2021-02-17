use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::vec::Vec;
use std::collections::HashMap;
use rand::{self, seq::SliceRandom, Rng};
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

    let mut output_file = setup_output(test_num);

    let item_names = read_input();

    let mut generated_items: HashMap<String, Package> = HashMap::new();

    for _ in 0..num_packages {
        let name = item_names.choose(&mut rand::thread_rng()).unwrap().clone();

        let package = match generated_items.get(&name) {
            // IF THAT GOOD HAS ALREADY BEEN PACKAGED, GET THAT PACKAGE AND ITS VALUES
            Some(t) => t.clone(),
            // IF NOT, THEN CREATE A PACKAGE
            None => {
                let weight = rand::thread_rng().gen_range(0..max_weight);
                let price = rand::thread_rng().gen_range(0..max_price);

                let temp_package = Package::new(name.clone(), weight, price);

                generated_items.insert(name, temp_package.clone());

                temp_package
            }
        };

        // WRITE BOX TO OUTPUT

        output_file.write_all(format!("{},{},{}\n",package.name(), package.weight(), package.price()).as_bytes())
            .expect("Cannot write to file");
    }
}

fn read_input() -> Vec<String> {
    //! Reads in all of the possible item names from the NAPCS list and outputs them as a vector

    let input_file = File::open("./NAPCS.txt").unwrap();
    let input_file = BufReader::new(input_file);

    let mut item_names = vec![];

    for (_, line) in input_file.lines().enumerate() {
        item_names.push(line.unwrap())
    }

    return item_names
}

fn setup_output(test_num: Option<u16>) -> File {
    let output_path = match test_num {
        Some(t) => format!("./available_packages/{}.csv", t),
        None => format!("./available_packages.csv")
    };

    let mut output_file = match File::create(&output_path) {
        Ok(t) => t,
        Err(_) => {
            // THIS IS GENERALLY BECAUSE THE FILE ALREADY EXISTS -- HANDLE IT BY DELETING SAID FILE
            // THEN CREATING IT
            std::fs::remove_file(&output_path).expect(&format!("Cannot remove the file at {}", output_path));
            File::create(&output_path).expect("Cannot create necessary output file")
        }
    };

    output_file.write_all("name,weight,price\n".as_bytes())
        .expect("Cannot write to file");

    output_file
}