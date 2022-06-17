use std::{collections::HashMap};

pub fn add_to_sales(name: String, employees: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let sales = employees.get("Sales"); 
    let mut updated_sales:Vec<String> = Vec::new();

    match sales {
        Some(list) => updated_sales = list.clone(),
        None => println!("None")
    };

    updated_sales.push(String::from(name));
    updated_sales.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    let mut updated_employees:HashMap<String, Vec<String>> = employees.clone();
    updated_employees.insert(String::from("Sales"), updated_sales);

    updated_employees
}