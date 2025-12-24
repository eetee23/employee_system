use std::{collections::HashMap, io::stdin};
use rand::Rng;

use crate::{Departments, Employee};

fn get_all_employees(employees: &HashMap<u32, Employee>) {
    for (k, v) in employees.iter() {
        println!("Id: {}\nName: {}\nAge: {}\nDepartment: {:?}\n-----------------\n", k, v.name, v.age, v.department);
    }
}

fn get_employee_by_name(employees: &HashMap<u32, Employee>) {
    let mut choice: String = String::new();
    println!("Enter employee name:");
    stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: String = choice.trim().parse().expect("Failed to parse");

    for (k, v) in employees.iter() {
        if v.name == choice {
            println!("Id: {}\nName: {}\nAge: {}\nDepartment: {:?}", k, v.name, v.age, v.department);
        }
    }
}

pub fn get_employees(employees: &HashMap<u32, Employee>) {
    let mut choice: String = String::new();

    println!("1. Get all employees");
    println!("2. Get employee by name");
    println!("Choose what you would like to do");

    stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Type a number");

    match choice {
        1 => get_all_employees(&employees),
        2 => get_employee_by_name(&employees),
        0 => return,
        _ => {
            println!("You type: {} this choice doesnt work", choice);
            return
        }
    }
}

pub fn add_employee(employees: &mut HashMap<u32, Employee>) {
    let id: u32 = loop {
        let mut rng: rand::prelude::ThreadRng = rand::rng();
        let id: u32 = rng.random_range(100_000..=999_999);
        if employees.contains_key(&id) {
            continue
        } else {
            break id;
        }
    };
    let mut name: String = String::new();
    let mut age: String = String::new();
    let mut department: String = String::new();

    println!("Enter employee name:");
    stdin().read_line(&mut name).expect("Failed to read line");
    let name: String = name.trim().parse().expect("Name coundn't be interpited");
    println!("Enter employee age:");
    let age: u8 =loop{
        stdin().read_line(&mut age).expect("Failed to read line");
        let age: u8 = match age.trim().parse::<u8>() {
            Ok(age) => age,
            Err(_) =>{
                println!("Type number");
                continue
                }
        };
        break age;
    };

    println!("1. Business");
    println!("2. HR");
    println!("3. Development");
    println!("4. Support");
    println!("5. Management");
    println!("6. Installation");
    println!("7. Warehouse");
    println!("Choose employee department:");
    let department: u8 = loop {
        stdin().read_line(&mut department).expect("Failed to read line");
        let department: u8 = match department.trim().parse::<u8>() {
            Ok(department) => department,
            Err(_) => {
                println!("Type number");
                continue
            }
        };
        break department;
    };

    let department: Departments = Departments::new(department);
    employees.entry(id).or_insert(Employee::new(name, age, department));
}