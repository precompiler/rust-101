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
    println!("{TWENTY_ONE_LOCAL} - {ONE_GLOBAL_SCOPE} = ?");
    // data types
    let a: i32 = 0;
    let b: bool = true;
    let c: char = 'x';
    let d/*:(i32, char, f32)*/ = (1, 'y', 1.1f32);
    let (x, y, z) = d;
    let x1 = d.0;
    let x2 = d.1;
    let x3 = d.2;
    println!("{a}, {b}, {c}, {:?}", d); // d is of type tuple, which is a compound type, and cannot be formatted using {}
    println!("{x}{y}{z}");
    println!("{x1}{x2}{x3}");

    let fib = [1, 1, 2, 3, 5]; // fixed length array
    println!("{}", fib[0]);
    let fib: [i32; 5] = [1, 1, 2, 3, 5]; // explicit typed
    println!("{:?}", fib);
    let dup1s = [1; 10]; // 10 1s in the array
    println!("{:?}", dup1s)
}
