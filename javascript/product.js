class Product {
    constructor(code, name, price) {
      this.code = code;
      this.name = name;
      this.price = price;
    }

    get get_code() {
      return this.code;
    }

    get get_name() {
        return this.name;
    }

    get get_price() {
        return this.price;
    }

  }
  
  const product = new Product(123, 'Aerofólio para o meu Palio', 250);