fn main() {
    type Integer = i32;
    let a: i32 = 1;
    let b: Integer = 1;
    println!("{}", a == b);
    println!("{}", a + b);

    type LongType = Box<dyn Fn() + Send + 'static>;
    let _x: LongType = Box::new(|| println!("Hello"));

    return_never_type();
}

fn return_never_type() -> ! {
    panic!("never return");
}