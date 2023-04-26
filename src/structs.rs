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
}

fn create_employee(id: i16, name: String, department: String) -> Employee {
    return Employee {
      id, name, department // assignment syntax sugar for name: name, department: department
    };
}