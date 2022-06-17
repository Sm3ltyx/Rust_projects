use std::{collections::HashMap};

pub fn add_to_engineering(name: String, employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let engineers = employees.get("Engineer"); 
    let mut updated_engineers:Vec<String> = Vec::new();

    match engineers {
        Some(list) => updated_engineers = list.clone(),
        None => println!("None")
    };

    updated_engineers.push(String::from(name));
    updated_engineers.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    let mut updated_employees:HashMap<String, Vec<String>> = employees.clone();
    updated_employees.insert(String::from("Engineer"), updated_engineers);

    updated_employees
}