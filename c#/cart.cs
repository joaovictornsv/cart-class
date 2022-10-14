namespace cartclass.csharp
{
    public class Cart
    {
        private int CartLimit {get; set;}
        public List<Product> Products { get; set;}

        public Cart(){
            products = new List<Product>();
        }

        public void AddProduct(Product product){
            Products.Add(product);
        }

        public void AddProduct(string code, string name, double price){
            AddProduct(new Product(code, name, price));
        }

        public void RemoveProduct(string code){
            foreach(Product x in Products){
                if(x.newCode == code) Products.remove(x);
            }
        }

        public Product SearchProduct(string code){
            foreach(Product x in Products){
                if(x.newCode == code) return x;
            }
            return null;
        }
    }
}