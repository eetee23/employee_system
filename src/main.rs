mod user_choice;
mod employee;

#[derive(Debug)]
pub enum Departments {
    Business,
    HR,
    Development,
    Support,
    Management,
    Installation,
    Warehouse,
    NoDepartment,
}

impl Departments {
    pub fn new(value: u8) -> Departments {
        match value {
            1 => Departments::Business,
            2 => Departments::HR,
            3 => Departments::Development,
            4 => Departments::Support,
            5 => Departments::Management,
            6 => Departments::Installation,
            7 => Departments::Warehouse,
            _ => Departments::NoDepartment,
        }
    }   
}

#[derive(Debug)]
pub struct Employee {
    name: String,
    age: u8,
    department: Departments
}

impl Employee {
    pub fn new(name: String, age: u8, department: Departments) -> Employee {
        Employee { name, age, department }
    }   
}

fn main() {
    use std::collections::HashMap;
    use std::io::stdin;
    use crate::user_choice::Choice;
    use crate::employee::{get_employees, add_employee};
    println!("Employee system");
    let mut employees: HashMap<u32, Employee> = HashMap::new();
    loop {
        println!("Enter choice for what you would like to do");
        println!("1. Enter employee");
        println!("2. Get all employees");
        println!("0. Exit");

        let mut choice: String = String::new();
        stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u8 = choice.trim().parse().expect("type number");

        let result: Choice = Choice::new(choice);

        match result {
            Choice::Exit => break,
            Choice::GetEmployees => get_employees(),
            Choice::AddEmployees => add_employee(&mut employees),
            Choice::InValidInput => {
                println!("Invalid input!");
                continue;
            }
        }
    }
    std::process::exit(0)
}
