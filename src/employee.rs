use std::{collections::HashMap, io::stdin};
use rand::Rng;

use crate::{Departments, Employee};


pub fn get_employees() {
    println!("get_employees")
}

pub fn add_employee(employees: &HashMap<u32, Employee>) {
    println!("add_employee");
    let mut rng = rand::rng();
    let id = rng.random_range(100_000..=999_999);

    let mut name: String = String::new();
    let mut age: String = String::new();
    let mut department: String = String::new();

    println!("Enter employee name:");
    stdin().read_line(&mut name).expect("Failed to read line");
    println!("Enter employee age:");
    loop{
        stdin().read_line(&mut age).expect("Failed to read line");
        let age: u8 = match age.trim().parse::<u8>() {
            Ok(age) => age,
            Err(_) =>{
                println!("Type number");
                continue
                }
        };
        break;
    }
    println!("1. Business");
    println!("2. HR");
    println!("3. Development");
    println!("4. Support");
    println!("5. Management");
    println!("6. Installation");
    println!("7. Warehouse");
    println!("Choose employee department:");
    loop {
        stdin().read_line(&mut department).expect("Failed to read line");
        let department: u8 = match department.trim().parse::<u8>() {
            Ok(department) => department,
            Err(_) => {
                println!("Type number");
                continue
            }
        };
        break;
    }

    



}