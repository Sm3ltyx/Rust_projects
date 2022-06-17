use std::{collections::HashMap};

pub fn add_to_programming(name: String, employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut updated_programmers = employees.get("Programmer").unwrap().clone(); 

    updated_programmers.push(String::from(name));
    updated_programmers.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    let mut updated_employees:HashMap<String, Vec<String>> = employees.clone();
    updated_employees.insert(String::from("Programmer"), updated_programmers);

    updated_employees
}