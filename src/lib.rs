/**
 * lib.rs is considered the entry of the library crate of the package.
 * A package can have multiple binary crates, but at most one library crate.
 */
mod pizza_house; // mod keyword is used to define a module, it doesn't mean current file is module 'pizza_house'
pub fn serve_pizza() {
    pizza_house::make_pizza();
}