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

    let msg = String::from("This is a msg");
    print_string(msg); // ownership of msg is moved to the function, when the scope of the function ends, msg will be freed
    // println!("{msg}");   // won't work as msg has been freed
    let i: i32 = 10;
    print_int32(i); // data types like i32 implements the `Copy` trait, which allows variables of such type to be stored in stack
    println!("{i}"); // print_int32 got a copy of i, the copy will be freed, but i itself is still valid here

    let mut msg = String::from("This is a msg");
    msg = print_string_and_return(msg); // the ownership passed to the function and returned back
    println!("{msg}");

    let msg = String::from("Another msg");
    let l = length(msg);
    println!("DEBUG: length is {l}");
    // println!("length of {msg} is {l}") // msg has been freed, as the ownership had been given to length function

    let msg = String::from("Another msg");
    let l = length_by_ref(&msg); // passing the reference of a variable allows a function to 'borrow' the value(in this example, the reference is read only), ownership won't change
    println!("length of '{msg}' is {l}");

    let mut msg = String::from("Another msg");
    let suffix = String::from("!!!");
    suffix_a_string(&mut msg, &suffix); // mutable reference
    println!("{msg}");

    let mut msg = String::from("Another msg");
    let _msg1 = &mut msg;
    let _msg2 = &mut msg;
    // println!("{msg1}, {msg2}"); // we cannot borrow a mutable reference more than once in the same scope
    suffix_a_string(&mut msg, &suffix);
    suffix_a_string(&mut msg, &suffix);
    let _msg1 = &msg;
    let _msg2 = &msg;
    let _msg3 = &mut msg; // we cannot have mutable reference if immutable refs alread exist
    println!("{msg}");
    // println!("{_msg1}");
    // let _s: String = "abc"; String literals are of type &str not String
    let _s: &str = "abc"; // String slice
    let _s: &str = &String::from("abc"); // a string slice should always be a reference
    let _s: &String = &String::from("abc");
    let _s1: &str = _s; // you can also assign a &String to &str
    let _s: &str = &(String::from("abc")[0..]);
    let ss = substring("hello abc", 0, 5);
    println!("{ss}");
    let s = &String::from("hello abc");
    let ss = substring(s, 0, 5);
    println!("{ss}");
    let ss = substring(&s[..], 0, 5);
    println!("{ss}");
    let _i: &[i32] = &[1, 2, 3][..]; // int slice
    println!("{:?}", _i)
}

fn print_string(s: String) {
    println!("{s}");
}

fn print_int32(i: i32) {
    println!("{i}");
}

fn print_string_and_return(s: String) -> String {
    println!("{s}");
    return s;
}

fn length(s: String) -> usize {
    return s.len();
}

fn length_by_ref(s: &String) -> usize {
    return s.len();
}

fn suffix_a_string(src: &mut String, suffix: &String) {
    src.push_str(suffix);
}
/*
fn invalid_string() -> &String {
    let ret = String::from("dummy"); // ownership of ret is this function, and ret will be freed when the function ends
    return &ret; // returning a reference to the outer scope is meaningless as the reference is pointing to a piece of freed memory, compiler will catch such issue
}
*/

fn substring(src: &str, start: usize, end: usize) -> &str {
    // to lazy to do sane checks
    return &src[start..end];
}