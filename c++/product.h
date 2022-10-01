#ifndef PRODUCT_H
#define PRODUCT_H
#include <string>
using std::string;

//Interface
class product{

public:
  product(string code, string name, double price);
  string getCode();
  string getName();
  double getPrice();
  void setCode(string newCode);
  void setName(string newName);
  void setPrice(double newPrice);
  string code;
  string name;
  double price;
};
#endif
