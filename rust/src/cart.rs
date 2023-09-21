use crate::product::Product;

use super::product;

#[derive(Clone, Debug)]
pub struct Cart {
    products: Vec<product::Product>,
}

impl Cart {
    pub fn new() -> Cart {
        return Cart {
            products: Vec::new()
        }
    }

    pub fn add_product(&mut self, product: &product::Product) {
        self.products.push(product.clone());
    }

    pub fn remove_product(&mut self, product_code: String) {
        match self.products.iter().position(|p| *p.code == product_code) {
            Some(position) => {
                self.products.remove(position);
                return ();
            }
            None => (),
        };
    }

    pub fn calculate_price(&self) -> f64 {
        let mut sum: f64 = 0.0;
        self.products.iter().for_each(|p| sum += p.price);

        return sum;
    }

    pub fn total_items(&self) -> usize {
        return self.products.len();
    }

    pub fn get_items(&self) -> Vec<product::Product> {
        return self.products.to_vec();
    }

    pub fn clear_cart(&mut self) {
        self.products = Vec::new();
    }

    // Metodo pra popular o cart com alguns produtos pra teste
    pub fn populate_cart(&mut self) {
        for i in 1..0 {
            let product = product::Product {
                code: i.to_string(),
                name: format!("Teste{}", i.to_string()), 
                price: 1000.00
            };
            self.products.push(product);
        }
    }
}
