fn main() {
    let /*mut*/ x = "test";
    // x.push_str(" it"); // string literal is immutable
    println!("{x}");
    let mut s = String::from("hello"); // allocated in heap
    s.push_str(" world");
    println!("{s}"); // when variable 's' is out of scope, memory allocated in heap will be dropped by rust

    let s1 = String::from("key");
    let s2 = s1; // in stack, s1 and s2 has the same value, which is the address of an allocated memory in heap
    // println!("{s1}"); s1.push_str(" x"); once we assign s1 to s2, s1 is considered invalid, we cannot use it anymore
    println!("{s2}");

    let s1 = String::from("key");
    let s2 = s1.clone(); // deep copy, value in heap will be copied, so s2 and s1 are pointing to different memories
    println!("{s1}, {s2}");
}