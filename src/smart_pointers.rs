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
}

#[derive(Debug)]
struct TreeNode<T> {
    data: T,
    // left: TreeNode<T>, compiler needs to know the size at compile time, but the size cannot be determined
    left: Box<Option<TreeNode<T>>>, // need to use Box, which is a smart pointer
    right: Box<Option<TreeNode<T>>>
}