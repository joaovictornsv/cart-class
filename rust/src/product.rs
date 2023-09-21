
#[derive(Clone, Debug)]
pub struct Product {
    pub code: String,
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new() -> Product {
        return Product {
            code: String::new(),
            name: String::new(),
            price: 0.0 
        }
    }    

    pub fn get_code(&self) -> String {
        return self.code.clone();
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_price(&self) -> f64 {
        return self.price;
    } 
 }
