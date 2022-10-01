class Product{
  public string $name;
  public string $code;
  public int $price;

  public function __construct(string $name, string $code, int $price)
    {
        $this->name = $name; 
        $this->code = $code; 
        $this->price = $price; 
    }

  public function getName(): string 
    {
        return $this->name;
    }

  public function getCode(): string 
    {
        return $this->code;
    }

  public function getPrice(): string 
    {
        return $this->price;
    }
}