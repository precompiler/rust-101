use std::convert::TryInto;
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
    println!("{:?}", dup1s);
    let sum = add(1, 2);
    println!("{sum}");
    let v = substract(5, 3);
    println!("{v}");
    let fib_ret = calc_fib_recursive(20);
    println!("{fib_ret}");
    dup_print_10_times(100);

    let idx = find(10, [1, 2, 3, 4, 5]);
    println!("{idx}");

    let idx = find(3, [1, 2, 3, 4, 5]);
    println!("{idx}");

    loop_label();
    while_loop();
    for_loop_array();
    for_range(8, false);
    for_range(8, true);
}

fn add(a: i32, b: i32) -> i32 {
    a + b //no semicolon indicates this is a expression not a statement, expression will have a return value
}

fn substract(a: i32, b: i32) -> i32 {
    return a - b;
}

fn calc_fib_recursive(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return calc_fib_recursive(n - 1) + calc_fib_recursive(n - 2);
    }
}

fn dup_print_10_times(n: i32) {
    let mut i = 0;
    loop {
        println!("{n}");
        i += 1;
        if i == 10 {
            break;
        }
    }
}

fn find(n: i32, t: [i32; 5]) -> i32 {
    let mut idx = -1;
    if t.len() == 0 {
        return idx;
    }
    let mut i = 0;
    idx = loop {
        if n == t[i] {
            break i.try_into().unwrap(); // break can return value for the loop
        }
        i += 1;
        if i == t.len() {
            break -1;
        }
    };
    return idx;
}

fn loop_label() {
    let mut i = 0;
    'outer_loop: loop {
        loop {
            println!("in loop");
            i += 1;
            if i > 5 {
                break 'outer_loop;
            }
        }
    }
}

fn while_loop() {
    let mut i = 0;
    while i < 3 {
        println!("{i}");
        i += 1;
    }
}

fn for_loop_array() {
    let x = [1, 2, 3, 4, 5];
    for n in x {
        println!("{n}");
    }
}

fn for_range(n: i32, reverse: bool) {
    if reverse {
        for i in (1..n).rev() {
            println!("({i}");
        }
    } else {
        for i in 1..n {
            println!("({i}");
        }
    }
}
