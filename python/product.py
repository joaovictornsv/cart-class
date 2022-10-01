from typing_extensions import Self


class Product:
    def __init__(self,code,name,price) -> None:
        self._code = code
        self._price = price 
        self._name = name

    #Code getter 
    @property
    def code(self):
        return self._code 

    #Code setter
    @code.setter 
    def code(self,cod):
        if not isinstance(cod,str):
            print('The code is wrong, try again!')
            return
        self._code = cod 