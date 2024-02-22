<?php
include "product.php";
class Cart
{
  public  $cartItems = [];
  function addItem(Product $product)
  {
    $this->cartItems[] = $product;
  }
  function removeLastItem()
  {
    array_pop($this->cartItems);
  }
  function clearCart()
  {
    if ($this->countItemsInCart() > 0) {
      for ($i = -1; $i <= $this->countItemsInCart(); $i++) {
        array_pop($this->cartItems);
      }
    }
  }
  function getItems()
  {
    return $this->cartItems;
  }
  function countItemsInCart()
  {
    return count($this->cartItems);
  }
  function getTotalPrice()
  {
    $price = 0;
    for ($i = 0; $i < $this->countItemsInCart(); $i++) {
      $price += $this->cartItems[$i]->getPrice();
    }
    return $price;
  }
  function removeItem(Product $product)
  {
    for ($i = 0; $i < $this->countItemsInCart(); $i++) {
      if ($product->getCode() == $this->cartItems[$i]->getCode()) {
        unset($this->cartItems[$i]);
        break;
      }
    }
  }
}

//Examples

// $product1 = new Product('test', 'test', 1);
// $product2 = new Product('test1', 'test2', 3);
// $product3 = new Product('test2', 'test3', 2);

// $cart = new Cart();

// $cart->addItem($product1);
// $cart->addItem($product2);
// $cart->addItem($product3);

// echo "Number of items in cart: " . $cart->countItemsInCart();
// echo "\nTotal price: " . $cart->getTotalPrice();
// echo "\nItems: ";
// echo print_r($cart->getItems());
// $cart->removeItem($product2);
// echo "\nItems (after removing product2): ";
// echo print_r($cart->getItems());
// $cart->clearCart();
// echo "\nItems (after clear): ";
// echo print_r($cart->getItems());
