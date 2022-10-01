#include <iostream>
#include <string>
#include "product.h"
#include "cart.h"
using std::cout, std::cin, std::endl;

int cart::limitCart = 100;
int cart::countProd = 0;

//Implementação
cart::cart(){ 
    products = new product[limitCart]
}

void cart::addProduct(string code, string name, double price){
    product p(code, name, price);
    addProduct(p);
}

cart cart::addProduct(const product &p){
    products[countProd] = p;
    countProd++;
    return *this;
} 

removeProduct product(string code){
  int pos = searchProduct(code);
  for(int i = pos; i < countProd; i++){
	  if (i < countProd){
		  products[i] = products[i+1];
	  }
	  else{
		  products[i] = product();
	  }
  countProd--;
	  
  }
}

int cart::searchProduct(string code) const{
  for(int i = 0; i < countProd; i++){
    if(products[i].getCode() == code){
      return i;
    }
  }
  return -1;
}

