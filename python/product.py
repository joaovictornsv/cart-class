from typing_extensions import Self


class Product:
    def __init__(self,code,name,price) -> None:
        self._code = code
        self._price = price 
        self._name = name