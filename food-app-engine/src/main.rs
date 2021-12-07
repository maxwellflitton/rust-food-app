use std::fs;

mod ingredients;
use ingredients::ingredients_map::IngredientsShopMap; // this will calculate what's needed for shopping lists


fn main() {
    println!("Hello, world!");
    let data = "Some data!";
    fs::write("./outcome.txt", data).expect("Unable to write file");
}
