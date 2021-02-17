mod utilities;
mod package;
mod truck;
mod warehouse;
mod knapsack;
use chrono::{DateTime, Utc};
use crate::warehouse::Warehouse;
use crate::truck::Truck;

fn main() {
    let x_min: u16 = 0;
    let x_max: u16 = 150;
    for i in x_min..x_max {
        utilities::set_creation::create_test_set(((i + 1) * 10) as usize, 100, 500, Some(i + 1));
    }

    evaluate_algorithms(x_min, x_max);
}

fn evaluate_algorithms(x_min: u16, x_max: u16) {
    for i in (x_min+1)..(x_max+1) {
        setup_algorithm(i)
    }
}

fn setup_algorithm(file_num: u16) {
    let start = chrono::Utc::now();

    let mut warehouse = Warehouse::new(format!("./available_packages/{}.csv", file_num));

    let original_value = warehouse.value();

    for i in 0..file_num {
        // if i % 10 == 1 or i == 0
        if i == 0 {
            let mut truck = Truck::new(((file_num + 1) * 10) as usize);
            //warehouse.add_truck(truck.clone());
            knapsack::dynamic_programming_solution(&mut warehouse, &mut truck);
        }
    }

    let loaded_value = original_value - warehouse.value();

    let duration = (chrono::Utc::now() - start).num_seconds();

    println!("{}: {}s | ${}", file_num, duration, loaded_value);
}
