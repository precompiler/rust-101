fn main() {
    let closure = |x| x;
    let _a = closure("abc"); // compiler got string ref type first
    // let _b = closure(1);  // fail as compiler refer the param type to be &str

    let add = |x: i32, y: i32| -> i32 { x + y };
    let mul = |x, y| { x * y };
    let r = calculator(1, 1, add);
    println!("{r}");
    let r = calculator(5, 5, mul);
    println!("{r}");

    let s = String::from("Hello");
    let func = || println!("{s}"); // closure doesn't take the ownership of s
    func();
    println!("{s}");

    let mut s = String::from("Hello");
    let mut func = || s.push_str(" world"); // mutable ref
    func();
    func();
    println!("{s}");

    let s = String::from("Hello");
    let func = move || println!("{s}"); // ownership of s moved to the closure
    func();
    // println!("{s}"); s has been released

    let x = vec![1, 2, 3];
    for i in x {
        println!("{i}");
    }

    let x = vec![1, 2, 3];
    for i in x.iter() {
        println!("{i}");
    }
    let mut it = x.iter();
    println!("{}", it.next() == Some(&1));
    let sum: i32 = x.iter().sum();
    println!("{}", sum);
    let sum: i32 = it.sum();
    println!("{}", sum);

    let x = vec![1, 2 ,3];
    let r: Vec<_> = x.iter().map(|x| x * x).collect();
    println!("{:?}", r);
}

fn calculator(a: i32, b: i32, func: fn(i32, i32) -> i32) -> i32 {
  return func(a, b);
}