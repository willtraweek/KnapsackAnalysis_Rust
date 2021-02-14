pub struct Package {
    name: String,
    weight: u16,
    price: u16
}

impl Package {
    pub fn new(name: String, weight: u16, price: u16) -> Package {
        Package {
            name,
            weight,
            price
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn weight(&self) -> weight {
        &self.weight
    }

    pub fn price(&self) -> weight {
        &self.price
    }
}