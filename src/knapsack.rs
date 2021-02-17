use crate::truck::Truck;
use crate::warehouse::Warehouse;
use crate::package::Package;

fn greedy_solution(warehouse: &mut Warehouse, truck: &mut Truck) {
    // ALWAYS LOADS THE MOST EXPENSIVE BOX INTO THE TRUCK
    while (warehouse.has_packages()) && (truck.leftover_space() > 0) {
        match warehouse.load_most_expensive_box(truck) {
            true => 0,  // DO NOTHING, THE OPERATION WAS A SUCCESS
            false => return // THE OPERATION IS COMPLETE, SINCE THERE ARE NO MORE BOXES TO LOAD
        }
    }
}