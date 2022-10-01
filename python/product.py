from typing_extensions import Self


class Product:
    def __init__(self,code,name,price) -> None:
        self._code = code
        self._price = price 
        self._name = name

    @property
    def code(self):
        return self._code 

    @code.setter 
    def code(self,cod):
        if not isinstance(cod,str):
            print('The code is wrong, try again!')
            return
        self._code = cod

    @property
    def price(self):
        return self._price

    @code.setter 
    def price(self,price_in):
        if not isinstance(price_in,str):
            print('The price is wrong, try again!')
            return
        self._price = price_in

