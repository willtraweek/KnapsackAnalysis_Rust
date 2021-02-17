use crate::truck::Truck;
use crate::warehouse::Warehouse;

pub fn greedy_solution(warehouse: &mut Warehouse, truck: &mut Truck) {
    // ALWAYS LOADS THE MOST EXPENSIVE BOX INTO THE TRUCK
    while (warehouse.has_packages()) && (truck.leftover_space() > 0) {
        let boxes_left =  warehouse.load_most_expensive_box(truck);
        if !boxes_left {
            return;  // THE OPERATION IS COMPLETE, SINCE THERE ARE NO MORE BOXES TO LOAD
        }
    }
}