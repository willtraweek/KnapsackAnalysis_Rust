use crate::truck::Truck;
use crate::warehouse::Warehouse;

pub fn dynamic_programming_solution(warehouse: &mut Warehouse, truck: &mut Truck) {
    let mut solution: Vec<Vec<Truck>> = Vec::new();
    for _ in 0..warehouse.num_packages() {
        let mut temp: Vec<Truck> = Vec::new();
        for _ in 0..truck.capacity() {
            temp.push(Truck::new(truck.capacity()));
        }
        solution.push(temp);
    }

    let packages = warehouse.to_vector();

    for x in 0..warehouse.num_packages() {
        for y in 0..truck.capacity() {
            if y < packages[x].weight() {
                // if the capacity couldn't fit this type of box, just keep the empty truck
                continue;
            }

            if x == 0 {
                // this means there is no previous data on the row above, so we'll have to assume that the current item
                // is the highest value for now
                solution[x][y].add_package(&packages[x].clone());
                continue;
            }

            let prev_max = &solution[x-1][y];
            // make a copy of the best possible truck that we could add the current item to
            let temp = &solution[x-1][y-packages[x].weight()];
            let mut curr_max = temp.clone();

            curr_max.add_package(packages[x]);

            if prev_max.value() < curr_max.value() {
                solution[x][y] = curr_max;
            } else {
                solution[x][y] = prev_max.clone();
            }
        }
    }

    let mut most_valuable_truck = solution[warehouse.num_packages()-1][truck.capacity()-1].clone();
    for (package, count) in most_valuable_truck.packages.iter() {
        for _ in 0..*count {
            warehouse.load_package(package, truck);
        }
    }
}

pub fn greedy_solution(warehouse: &mut Warehouse, truck: &mut Truck) {
    // ALWAYS LOADS THE MOST EXPENSIVE BOX INTO THE TRUCK
    while (warehouse.has_packages()) && (truck.leftover_space() > 0) {
        let boxes_left =  warehouse.load_most_expensive_box(truck);
        if !boxes_left {
            return;  // THE OPERATION IS COMPLETE, SINCE THERE ARE NO MORE BOXES TO LOAD
        }
    }
}