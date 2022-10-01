#ifndef CART_H
#define CART_H
#include <string>
#include "product.h"
using std::string;

//Interface
class cart{

public:
  cart();
  void addProduct(string, string, double);
  cart addProduct(const product &);
  void removeProduct(string);
  int searchProduct(string) const;

protected:
  product *products
};

#endif
