mod fridge;
pub fn make_pizza() {
    fridge::get_ingredients();
    println!("making a pizza")
}