use std::ops::Add;

fn main() {
    let mut c = Counter{c: 10};
    println!("{} {} {} {}", c.next().unwrap(), c.next().unwrap(), c.next().unwrap(), c.next().unwrap());

    let mut c = 10;
    println!("{} {} {} {}", c.next().unwrap(), c.next().unwrap(), c.next().unwrap(), c.next().unwrap());

    let mut c = Counter2{_c: 1};
    println!("{}", <Counter2 as GenericIterator<String>>::next(&mut c).unwrap());

    let cn1 = ComplexNumber{r: 1.0, i: 2.0};
    let cn2 = ComplexNumber{r: 2.0, i: 3.0};
    let cn_sum = cn1 + cn2;
    println!("{} + {}i", cn_sum.r, cn_sum.i);
    let cn3 = ComplexNumber{r: 3.0, i: 1.0};
    let cn_sum2 = cn3 + 3.0;
    println!("{} + {}i", cn_sum2.r, cn_sum2.i);

    let player = SpotifyPlayer;
    player.play();
    DVDPlayer::play(&player);
    MP3Player::play(&player);

    JVM::shut_down();
    <JVM as PowerManager>::shut_down();
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

struct ComplexNumber {
    r: f32,
    i: f32
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, rhs: Self) -> Self::Output {
        return ComplexNumber {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl Add<f32> for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, rhs: f32) -> Self::Output {
        return ComplexNumber {
            r: self.r + rhs,
            i: self.i
        }
    }
}

trait DVDPlayer {
    fn play(&self);
}

trait MP3Player {
    fn play(&self);
}

struct SpotifyPlayer;

impl SpotifyPlayer {
    fn play(&self) {
        println!("Spotify play...")
    }
}

impl DVDPlayer for SpotifyPlayer {
    fn play(&self) {
        println!("DVD play...");
    }
}

impl MP3Player for SpotifyPlayer {
    fn play(&self) {
        println!("MP3 play...");
    }
}

trait PowerManager {
    fn shut_down();
}

struct JVM;
impl JVM {
    fn shut_down() {
        println!("shutting down JVM");
    }
}

impl PowerManager for JVM {
    fn shut_down() {
        println!("shutting down JVM and turning off PC");
    }
}

trait Displayable {
    fn display(&self);
}

trait ThreeD : Displayable {
    fn render(&self);
}

struct Sprite {
    x: f32,
    y: f32
}
impl Displayable for Sprite {
    fn display(&self) {
        println!("{} {}", self.x, self.y);
    }
}
impl ThreeD for Sprite {
    fn render(&self) {
        self.display();
    }
}