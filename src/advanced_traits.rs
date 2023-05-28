

fn main() {
    let mut c = Counter{c: 10};
    println!("{} {} {} {}", c.next().unwrap(), c.next().unwrap(), c.next().unwrap(), c.next().unwrap());

    let mut c = 10;
    println!("{} {} {} {}", c.next().unwrap(), c.next().unwrap(), c.next().unwrap(), c.next().unwrap());

    let mut c = Counter2{_c: 1};
    println!("{}", <Counter2 as GenericIterator<String>>::next(&mut c).unwrap());
}

trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    c: i32
}

struct Counter2 {
    _c: i32
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.c >= 0 {
            let ret = Some(self.c);
            self.c -= 1;
            return ret
        } else {
            return None;
        }
    }
}

impl Iterator for i32 {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if *self >= 0 {
            let ret = Some(self.clone());
            *self -= 1;
            return ret
        } else {
            return None;
        }
    }
}

impl GenericIterator<i32> for Counter2 {
    fn next(&mut self) -> Option<i32> {
        return Some(100);
    }
}

impl GenericIterator<String> for Counter2 {
    fn next(&mut self) -> Option<String> {
        return Some(String::from("hello"));
    }
}