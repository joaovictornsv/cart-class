import java.util.ArrayList;

public class Cart {
    ArrayList<Product> products = new ArrayList<>();

	public Cart() {

	}

    public void add_product(Product product) {
		products.add(product);
	}

	public void remove_product(String product_code) {
		for (int i = 0; i < products.size(); i++) {
			if (products.get(i).getCode() == product_code) {
				products.remove(i);
			}
		}
	}

	public Double calculate_price() {
		Double price = 0.0;
		
		for (int i = 0; i < products.size(); i++) {
			price += products.get(i).getPrice();
		}
		
		return price;
	}
	
	public Integer total_itens() {
		return products.size();
	}
	
	public ArrayList<Product> get_itens() {
		return products;
	}
	
	public void clear_cart() {
		products.clear();
	}
	

}
