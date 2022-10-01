class Product {
  late String _name;
  late String _code;
  late int _price;

  Product(String name, String code, int price) {
    this.code = code;
    this.name = name;
    this.price = price;
  }

  String get name => _name;
  set name(String name) => _name = name;

  String get code => _code;
  set code(String code) => _code = code;

  int get price => _price;
  set price(int price) => _price = price;
}
