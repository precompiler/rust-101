use std::collections::HashMap;

#[derive(Debug)]
enum MultiTypes {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let _v: Vec<i8> = Vec::new();
    // _v.push(1); immutable
    let mut _v: Vec<i8> = Vec::new();
    _v.push(1);
    println!("{:?}", _v);
    let _v = vec![1, 2, 3, 4]; // vec macro
    let e = &_v[0];
    println!("{e}");
    let e = _v.get(0);
    match e {
        Some(x) => println!("{x}"),
        None => println!("idx out of bound")
    }

    let e = _v.get(10);
    match e {
        Some(x) => println!("{x}"),
        None => println!("idx out of bound")
    }

    let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];
    println!("{:?}", v);
    for i in &mut v {
        *i = *i * 10;
    }
    println!("{:?}", v);
    let v = vec![ // a way of using vec to store elements of different types
        MultiTypes::Int(10),
        MultiTypes::Float(1.1),
        MultiTypes::Text(String::from("Hello"))
    ];
    println!("{:?}", v);

    // a string is a collection of bytes
    let mut s = String::new();
    s.push('h');
    let s2 = "ello";
    s.push_str(s2);
    println!("{s}");
    println!("{s2}");

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{s3}");
    // println!("{s1}"); '+' operator uses the 'add' method of String, the ownership of s1 has been given to s3
    println!("{s2}");

    let s1 = "a";
    let s2 = "b";
    let s3 = "c";
    let s4 = format!("{s1} {s2} {s3}"); // format macro won't mess with variables ownership
    println!("{s4}");
    println!("{s1} {s2} {s3}");

    let msg = "abcdefg";
    for c in msg.chars() {
        println!("{c}");
    }
    for c in msg.bytes() {
        println!("{c}");
    }

    let mut cache = HashMap::new();
    cache.insert("one", 1);
    cache.insert("two", 2);
    cache.entry("two").or_insert(-1); // insert if not present
    let v = cache.entry("three").or_insert(2);
    *v = *v + 1; // update existing value
    // cache.insert("three", "3"); values should have the same type
    println!("{:?}", cache);
    let two = cache.get("two").expect("expected value not found");
    println!("{two}");
    for (key, value) in &cache { // need to use ref as into_iter() will be implicitly called, which takes away ownership of cache
        println!("{key} => {value}");
    }
    println!("{:?}", cache);
}