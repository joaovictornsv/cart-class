import {faker} from '@faker-js/faker';
import {Product} from "./product";
import {Cart} from "./cart";


console.log("Creating fake objects")


const product = new Product(faker.commerce.product(), faker.commerce.productName(), faker.datatype.number().toString());
console.log("Random product: " + JSON.stringify(product))

const cart = new Cart([product])
console.log("Random cars: " + JSON.stringify(cart))