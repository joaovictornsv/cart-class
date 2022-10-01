#include <iostream>
#include <string>
#include "product.h"
using std::cout, std::cin, std::endl;

//Implementação
product::product(string newCode, string newName, double newPrice){ 
  setCode(newCode);
  setName(newName);
  setPrice(newPrice);
}

void product::setCode(string newCode){ 
    code = newCode;      
}

void product::setName(string newName){ 
    name = newName;      
}

void product::setPrice(double newPrice){ 
    price = newPrice;      
}

string product::getCode(){
	return code;
}

string product::getName(){
	return name;
}

double product::getPrice(){
	return price;
}




