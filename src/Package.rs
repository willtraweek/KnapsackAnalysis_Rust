#[derive(Clone)]
pub struct Package {
    name: String,
    weight: usize,
    price: usize
}

impl Package {
    pub fn new(name: String, weight: usize, price: usize) -> Package {
        Package {
            name,
            weight,
            price
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn weight(&self) -> usize {
        self.weight
    }

    pub fn price(&self) -> usize {
        self.price
    }
}