static _KEY: i32 = 1;
fn main() {
    let mut num = 5;
    let n1 = &num as *const i32;
    let n2 = &mut num as *mut i32;
    unsafe {
        println!("n1 {}", *n1);
        println!("n2 {}", *n2);
        *n2 = 100;
        println!("n1 {}", *n1);
        println!("n2 {}", *n2);
    }
    let p: *const i32;
    {
        let x = 1;
        p = &x as *const i32;
        unsafe {
            println!("{}", *p);
        }
    }
    unsafe {
        println!("{}", *p);
    }

    unsafe {
        unsafe_func();
    }

    // unsafe {
    //     println!("{}", abc(-1));
    // }
}

unsafe fn unsafe_func() {}

// extern "C" {
//     fn abc(input: i32) -> i32;
// }

unsafe trait Dummy {}

unsafe impl Dummy for String {}