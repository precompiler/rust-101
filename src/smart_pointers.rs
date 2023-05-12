use std::ops::Deref;

fn main() {
    let mut x = 1;
    let p = &mut x;
    *p = 2;
    println!("{p}");
    println!("{x}");

    let _x = 1; // x is stored on stack
    let _x = Box::new(1); // x is stored on heap

    let _node1 = TreeNode{data: 1, left: Box::new(None), right: Box::new(None)};
    let _node2 = TreeNode{data: 2, left: Box::new(None), right: Box::new(None)};
    let root = TreeNode{data: 0, left: Box::new(Some(_node1)), right: Box::new(Some(_node2))};
    println!("node data => {:?}, right => {:?}, left => {:?}", root.data, root.right, root.left);

    let mut x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5); // need to deref y
    x = 6;
    assert_eq!(x, 6);
    // assert_eq!(*y, 5); // y borrows x, if y is still in scope, we won't be able to

    let mut x = 5;
    let y = Box::new(x); // y uses a copy of x
    x = 6; // we can still modify x while y is still in scope as y is not referencing x
    assert_eq!(x, 6);
    assert_eq!(*y, 5); // need to deref y

    let mut x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    x = 10;
    assert_eq!(x, 10);
    assert_eq!(*y, 5);

    let mut s = String::from("Hello");
    let y = MyBox::new(s.clone());
    println!("{s} {}", y.x);
    s.push_str(" World");
    println!("{s} {}", y.x);

    let x = 9;
    let y = 9;
    let sum = add(&CrazyBox::new(x), &CrazyBox::new(y));
    println!("{sum}");

    let x = String::from("abc");
    let y = String::from("def");
    let sum = add(&CrazyBox::new(x), &CrazyBox::new(y)); // x, y don't match the param type of add function,
    // but since CrazyBox implements Deref, rust will call deref method to convert a reference to a type to a reference to another type
    println!("{sum}");

}

#[derive(Debug)]
struct TreeNode<T> {
    data: T,
    // left: TreeNode<T>, compiler needs to know the size at compile time, but the size cannot be determined
    left: Box<Option<TreeNode<T>>>, // need to use Box, which is a smart pointer
    right: Box<Option<TreeNode<T>>>
}

struct MyBox<T>{x: T}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox { x };
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.x;
    }
}

struct CrazyBox<T>{_x: T}

impl<T> CrazyBox<T> {
    fn new(_x: T) -> CrazyBox<T> {
        return CrazyBox{_x};
    }
}

impl<T> Deref for CrazyBox<T> {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        return &1;
    }
}

fn add<'a> (a: &'a i32, b: &'a i32) -> i32 {
    return a + b;
}