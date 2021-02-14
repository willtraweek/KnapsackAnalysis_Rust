use crate::package::Package as Package;
use std::collections::HashMap;

pub struct Truck {
    //! Truck to be filled with packages
    //!
    //! Attributes:
    //! * capacity: the maximum weight a truck can hold
    //! * weight: the current weight
    //! * value: the value of all of the boxes in the truck
    //! * boxes: A map of all of the boxes that are in the truck
    capacity: u32,
    weight: u32,
    value: u32,
    boxes: HashMap<String, u16>
}

