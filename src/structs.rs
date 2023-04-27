struct Employee {
    id: i16,
    name: String,
    department: String
}

// tuple structs
struct RGBColor (u8, u8, u8); // default order is R G B, no need to give each field a name
struct _3DPosition (f32, f32, f32); // x, y, z

// struct with no field
struct Nil;

fn main() {
    let name = String::from("Scott Tiger");
    let mut employee = Employee{
        id: 0,
        name: name,
        department: String::from("Database")
    };
    println!("{}, {}", employee.name, employee.department);
    employee.department = String::from("Sales");
    println!("{}, {}", employee.name, employee.department);
    let another_emp = create_employee(employee.id, employee.name.clone(), employee.department.clone());
    println!("{}, {}", another_emp.name, another_emp.department);
    println!("{}, {}", employee.name, employee.department);
    let another_emp = Employee{
        name: String::from("John Doe"),
        ..employee  // initialize a struct instance using another instance
    };
    println!("{}, {}", another_emp.name, another_emp.department);
    println!("{}", employee.id); // id is of type i16, still can be used
    // println!("{}, {}", employee.name, employee.department); // ownership of name and department have been given to another_emp

    let _red = RGBColor(255, 0, 0);
    let _nil = Nil{};

    let mut rect = Rectangle{width: 10, height: 10};
    println!("rect info {:?}, area is {}", rect, rect.area());
    rect.to_black_hole();
    println!("rect info {:?}, area is {}", rect, rect.area());

    let r1 = Rectangle{width: 1, height: 1};
    let r2 = Rectangle{width: 2, height: 2};
    let r3 = Rectangle{width: 3, height: 3};
    println!("r1 <> r2 = {}", Rectangle::compare_area(&r1, &r2));
    println!("r2 <> r1 = {}", Rectangle::compare_area(&r2, &r1));
    println!("r3 <> r3 = {}", Rectangle::compare_area(&r3, &r3));
}

fn create_employee(id: i16, name: String, department: String) -> Employee {
    return Employee {
      id, name, department // assignment syntax sugar for name: name, department: department
    };
}

#[derive(Debug)]
struct Rectangle {
    width: i16,
    height: i16
}

impl Rectangle {
    // method
    fn area(&self/*short hand for self: &Self*/) -> i16 {
        return self.width * self.height;
    }
    fn to_black_hole(&mut self) {
        self.height = 0;
        self.width = 0;
    }
    // associated function
    fn compare_area(r1: &Rectangle, r2: &Rectangle) -> i8 {
        let a1 = r1.width * r1.height;
        let a2 = r2.width * r2.height;
        let x = a1 - a2;
        return if x > 0 {
            1
        } else if x < 0 {
            -1
        } else {
            0
        };
    }
}