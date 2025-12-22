mod user_choice;
mod employee;

pub enum Departments {
    Business,
    HR,
    Development,
    Support,
    Management,
    Installation,
    Warehouse,
}

pub struct Employee {
    name: String,
    age: u8,
    department: Departments
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
            Choice::AddEmployees => add_employee(&employees),
            Choice::InValidInput => {
                println!("Invalid input!");
                continue;
            }
        }
    }
    std::process::exit(0)
}
