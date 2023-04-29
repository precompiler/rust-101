enum Color {
    Red,
    Green,
    _Blue
}

struct Flower {
    _color: Color,
    _name: String
}

enum Colors {
    Of(String)
}

enum Flowers {
    Of(Flower)
}
#[derive(Debug)]
enum SystemEvent {
    Quit,
    MoveWindowTo {_x: i32, _y: i32},
    Print(String),
    ChangeBGColor(u8, u8, u8)
}

impl SystemEvent {
    fn log_event(&self) {
        println!("{:?}", self);
    }
    fn process_event(evt: SystemEvent) {
        match evt {
            SystemEvent::Quit => println!("Quit system..."),
            SystemEvent::MoveWindowTo{_x, _y} => println!("Moving window to {}:{}", _x, _y),
            SystemEvent::Print(msg) => println!("Print msg => {}", msg),
            SystemEvent::ChangeBGColor(r, g, b) => println!("Change bg color to {},{},{}", r, g, b)
        }
    }
}

fn square(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * i)
    }
}

fn main() {
    let _red = Color::Red;
    let _green = Color::Green;
    let _f1 = Flower{
        _color: Color::Red,
        _name: String::from("f1")
    };
    let _f = Flowers::Of(_f1);
    let _pink = Colors::Of(String::from("pink"));
    let event = SystemEvent::Quit;
    event.log_event();
    let event = SystemEvent::MoveWindowTo {_x: 10, _y: 10};
    event.log_event();
    SystemEvent::process_event(event);
    let event = SystemEvent::Print(String::from("Hello!"));
    event.log_event();
    SystemEvent::process_event(event);
    let event = SystemEvent::ChangeBGColor(100, 100, 100);
    event.log_event();
    SystemEvent::process_event(event);
    let x = Some(2);
    let y = square(x);
    println!("{}", y.expect("error"));
    let x = None;
    let y = square(x);
    println!("{}", y.is_none());
}
