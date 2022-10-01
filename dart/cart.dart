import 'product.dart';

class Cart {
  late List<Product> products;

  void addProduct(Product product) {
    this.products.add(product);
  }

  void removeProduct(String productCode) {
    for (int i = 0; i < products.length; i++) {
      if (products[i].code == productCode) {
        products.removeAt(i);
      }
    }
  }

  double calculatePrice() {
    double total = 0;
    for (int i = 0; i < products.length; i++) {
      total = total + products[i].price;
    }
    return total;
  }

  int totalItens() {
    return products.length;
  }

  List<Product> getItens() {
    return products;
  }

  void clearCart() {
    products.clear();
  }
}
