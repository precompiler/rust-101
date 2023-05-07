fn main() {
    let l = vec![1, 2, 10, 80, 3];
    let _x: &[i32] = &l; // ref to a vector can be assigned to ref to an array as well.
    let _x: &Vec<i32> = &l;
    let m = max_i32(&l);
    println!("max number in {:?} is {}", l, m);

    let l = [1, 3, 8,2];
    let m = max_i32(&l);
    println!("max number in {:?} is {}", l, m);

    let l = ['a', 'z', 'c', 'b'];
    let m = max(&l);
    println!("max char in {:?} is {}", l, m);

    let l = [1.0, 2.8, 3.2, -0.1];
    let m = max(&l);
    println!("max float in {:?} is {}", l, m);

    let _p = ThreeDPoint{_x: 1, _y: 2, _z: 1.5};
    println!("{:?}", _p);

    let p1 = Point{x: 1, y: 2};
    let p2 = Point{x: '3', y: '4'};
    let p3 = p1.dummy(p2);
    println!("{}, {}", p3.x, p3.y);
    // println!("{:?}", p1); p1 has been dropped
}

fn max_i32(list: &[i32]) -> &i32 {
    let mut max = &list[0];
    for i in list {
        if i > max {
            max = i;
        }
    }
    return max;
}

fn max<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for i in list {
        if i > max {
            max = i;
        }
    }
    return max;
}
#[derive(Debug)]
struct ThreeDPoint<X, Y, Z> {
    _x: X,
    _y: Y,
    _z: Z
}
#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y
}

impl<X, Y> Point<X, Y> {
    fn dummy<X1, Y1>(self, p: Point<X1, Y1>) -> Point<X, Y1> {
        return Point{x: self.x, y: p.y};
    }
}