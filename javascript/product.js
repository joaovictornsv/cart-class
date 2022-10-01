class Product {
    constructor(code, name, price) {
      this.code = code;
      this.name = name;
      this.price = price;
    }
    // Getter
    get get_code() {
      return this.code;
    }

    get get_name() {
        return this.name;
    }

    get get_price() {
        return this.price;
    }
    // Method
  }
  
  const product = new Product(123, 'Aerofólio para o meu Palio', 250);
  
  console.log(product.get_code); //123
  console.log(product.get_name); //Aerofólio para o meu Palio
  console.log(product.get_price); //250