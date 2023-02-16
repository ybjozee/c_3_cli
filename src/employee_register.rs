use std::collections::HashMap;

use crate::input_processor::{get_int_from_input, read_input};

pub fn start() {
    print_instructions();

    let mut organization: HashMap<String, Vec<String>> = HashMap::new();

    let mut user_input = read_input();

    while user_input.trim().ne("") {
        let (employee_name, employee_department) = get_employee_and_department_from_input(user_input);
        let department = organization.entry(employee_department).or_insert(Vec::new());
        department.push(employee_name);
        user_input = read_input();
    }

    print_closing_instructions();

    let mut user_input = get_int_from_input();

    while user_input != 3 {
        match user_input {
            1 => {
                println!("Enter department name");
                let selected_department = read_input();
                print_employees(organization.entry(selected_department).or_default());
            }
            2 => {
                for (department, employees) in &organization {
                    println!("{department} departmnent");
                    print_employees(&employees);
                    println!("========================")
                }
            }
            _ => {
                print_closing_instructions();
            }
        }

        print_closing_instructions();
        user_input = get_int_from_input();
    }
}

fn print_closing_instructions() {
    println!("Press 1 to view employees for a department");
    println!("Press 2 to view employees for all departments");
    println!("Press 3 to exit register");
}

fn print_instructions() {
    println!("Welcome to the Rustacean employee register");
    println!("Add an employee to a department by typing \"Add *employee* to *department*\"");
    println!("Press enter to stop entering");
}

fn get_employee_and_department_from_input(input: String) -> (String, String) {
    let words: Vec<&str> = input.split(' ').collect();
    (String::from(words[1]), String::from(words[3]))
}

fn print_employees(employees: &Vec<String>) {
    for employee in employees {
        println!("{employee}");
    }
}

