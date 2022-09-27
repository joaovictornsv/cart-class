# Classe Carrinho

Repositório que contém a implementação da famosa classe "Carrinho de Compras" em diversas linguagens.

## ➡ Implementação
A classe e seus métodos devem seguir os seguintes contratos:

### class: Product 📦
```
attributes:
- code: string    // Código do produto
- name: string    // Nome do produto
- price: float    // Preço do produto

methods:
- get_code() -> string    // Retorna código do produto
- get_name() -> string    // Retorna nome do produto
- get_price() -> float   // Retorna preço do produto

Obs: se preferir pode utilizar setters e getters nativos da sua linguagem ao invés dos métodos get_xxx, 
```


### class: Cart 🛒
```
attributes:
- products: Product[]   // Lista de produtos

methods:

- add_product(product: Product) -> void         // Adiciona produto ao carrinho
- remove_product(product_code: string) -> void  // Remove produto pelo código
- calculate_price() -> float                    // Calcula preço total do carrinho
- total_itens() -> int                          // Retorna o total de itens do carrinho
- get_itens() -> Product[]                      // Retorna uma cópia da lista de produtos
- clear_cart() -> void                          // Limpa a lista de produtos
```

## ➡ Organização dos arquivos
Se deseja submeter uma contribuição, crie uma pasta com o nome da linguagem utilizada e adicione dois arquivos: `cart` e `product`.
Exemplo:
```
- cart-class
    python
        | cart.py
        | product.py
    cpp
        | cart.cpp
        | product.cpp
    README.md <- você está aqui
```
