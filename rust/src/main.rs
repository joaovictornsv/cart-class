mod product;
mod cart;

fn main() {
    // Pra testar e ver os logs é só rodar cargo build e executar
    // o binário

    let mut cart = cart::Cart::new();
    let mut product = product::Product::new();
    product.name = String::from("Processador"); 
    product.price = 1000.00;
    product.code = String::from("123123"); 
    println!("{:?}\n", product);

    // Popula carrinho com dados de teste
    cart.populate_cart();

    // Testes Cart
    cart.add_product(&product);
    println!("{:?}\n", cart);

    cart.remove_product(product.code.clone());
    println!("{:?}\n", cart);

    // Adiciona dois Products pra testar a soma: deve resultar em 2000
    cart.add_product(&product);
    cart.add_product(&product);
    let price = cart.calculate_price();
    println!("{:?}\n", price);

    let size = cart.total_items();
    println!("{:?}\n", size);

    let items = cart.get_items();
    println!("{:?}\n", items);

    cart.clear_cart();
    println!("{:?}\n", cart);
}
