use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;
use crate::List::{Cons, Nil};
use std::cell::RefCell;

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

    {
        let _a = DummyDroppable{data: String::from("a")};
        let _b = DummyDroppable{data: String::from("b")}; // last defined, first dropped
    }

    let _a = DummyDroppable{data: String::from("drop before scope ends")};
    drop(_a);
    println!("_a dropped before scope ends");

    let a = Rc::new(Cons(3, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("reference count {}", Rc::strong_count(&a));
    let b = Cons(4, Rc::clone(&a)); // Rc::clone won't deep clone a, it just increases the reference counter
    println!("reference count {}", Rc::strong_count(&a));
    let c = Cons(5, Rc::clone(&a));
    println!("reference count {}", Rc::strong_count(&a));
    println!("{:?} {:?}", b, c);
    println!("{:?}", a);

    let _a = Box::new(String::from("a"));
    {
        let _b = Box::new(&a);

    }
    let _c = Box::new(&a);
    println!("{:?}", _a);
    //println!("{:?}", _b);
    println!("{:?}", _c);

    test_switcher();
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

struct DummyDroppable {
    data: String
}

impl Drop for DummyDroppable {
    fn drop(&mut self) {
        println!("about to drop the DummyDroppable, data => {}", self.data);
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}

trait Messenger {
    fn send(&self, msg: &str);
}

struct Switcher<'a, T: Messenger> {
    messenger: &'a T,
    channel: u8
}
impl<'a, T> Switcher<'a, T>
where
  T: Messenger {
    fn new(messenger: &'a T, channel: u8) -> Switcher<'a, T> {
        return Switcher {messenger, channel};
    }

    fn set_channel(&mut self, channel: u8) {
        self.channel = channel;
        if self.channel == 0 {
            self.messenger.send("use default channel");
        } else if self.channel == 1 {
            self.messenger.send("use channel 1");
        } else {
            self.messenger.send("unknown channel");
        }
    }
}

#[derive(Debug)]
struct MockMessenger {
    msg_cache: RefCell<Vec<String>>
}
impl MockMessenger {
    fn new() -> MockMessenger {
        return MockMessenger {
            msg_cache: RefCell::new(vec![])
        }
    }
}
impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.msg_cache.borrow_mut().push(String::from(msg)); // even self is immutable, we can still mutate msg_cache field, as it's a RefCell.
        // self.msg_cache.borrow_mut() // will panic, we cannot borrow it twice;
    }
}
fn test_switcher() {
    let mock_messenger = MockMessenger::new();
    let mut switcher = Switcher::new(&mock_messenger, 0);
    switcher.set_channel(1);
    switcher.set_channel(2);
    println!("{:?}", mock_messenger)
}