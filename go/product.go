package product

type Product struct {
	code  string
	name  string
	price float64
}

func new_product(code string, name string, price float64) Product {
	return Product{code: code, name: name, price: price}
}

func (p Product) get_code() string {
	return p.code
}

func (p Product) get_name() string {
	return p.name
}

func (p Product) get_price() float64 {
	return p.price
}