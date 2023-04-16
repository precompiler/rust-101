// constant declared in global scope
// type must be specified
const ONE_GLOBAL_SCOPE: u16 = 1;

fn main() {
    // local-scoped constant
    const TWENTY_ONE_LOCAL: u16 = 21;
    let i = 1;
    // i = 2; // error! variables are immutable by default
    let i = i + 1; // but you can redefine it
    {
        let i = 1000;
        println!("inner scoped i = {i}");
        let i = "a message"; // can have different data types when redefined
        println!("i as a String => {i}")
    }
    println!("outter scoped i = {i}");
    let mut x = 1;
    x += i;
    println!("x = {x}");
    println!("{TWENTY_ONE_LOCAL} - {ONE_GLOBAL_SCOPE} = ?")
}
