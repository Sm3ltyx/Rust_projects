/*• Using a hash map and vectors, create a text interface to allow a user
to add employee names to a department in a company. For example,
“Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
a list of all people in a department or all people in the company by department,
sorted alphabetically.*/

mod department;

use std::{io, collections::{HashMap}, io::Write};
use colored::Colorize;

pub use crate::department::*;

pub fn main() {
    //START CONFIG
    let engineers: Vec<String> = Vec::new();
    let sales: Vec<String> = Vec::new();
    let programmer: Vec<String> = Vec::new();
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    employees.insert(String::from("Engineer"), engineers );
    employees.insert(String::from("Sales"), sales);
    employees.insert(String::from("Programmer"), programmer);

    //USER INPUT
    println!("{}", "Commands:\n- add 'employee' to 'department'\n- list 'department' (or 'all')\n- departments\n- help\n- quit\n".blue());
    loop {
        print!("company_manager % ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Please type a valid command");
        let mut command_words: Vec<String> = Vec::new();
        for word in command.split_whitespace() {
            command_words.push(String::from(word));
        }
        if command_words.len() == 4 && command_words[0] == "add" && command_words[2] == "to"{
            if command_words[3] == "Engineer" {
                employees = department::engineering::add_to_engineering(String::from(&command_words[1]), employees);
                println!("{}", "Command accepted.".green());
            } else if command_words[3] == "Sales" {
                employees = department::sales::add_to_sales(String::from(&command_words[1]), employees);
                println!("{}", "Command accepted.".green());
            } else if command_words[3] == "Programmer" {
                employees = department::programming::add_to_programming(String::from(&command_words[1]), employees);
                println!("{}", "Command accepted.".green());
            } else {
                println!("{}", "Command not valid.".red());
            }
        } else if command_words.len() == 2 && command_words[0] == "list" {
            if command_words[1] == "Engineer" {
                println!("{}", "Command accepted.".green());
                for (key, value) in &employees {
                    for people in value {
                        if key == "Engineer" {
                            println!("{}", people);
                        }
                    }
                }
            } else if command_words[1] == "Sales" {
                println!("{}", "Command accepted.".green());
                for (key, value) in &employees {
                    for people in value {
                        if key == "Sales" {
                            println!("{}", people);
                        }
                    }
                }
            } else if command_words[1] == "Programmer" {
                println!("{}", "Command accepted.".green());
                for (key, value) in &employees {
                    for people in value {
                        if key == "Programmer" {
                            println!("{}", people);
                        }
                    }
                }
            } else if command_words[1] == "all" {
                println!("{}", "Command accepted.".green());
                for (key, value) in &employees {
                    println!("{}", key);
                    for people in value {
                            println!(" {}", people);
                    }
                }
            }else {
                println!("{}", "Command not valid.".red());
            }
        } else if command_words[0] == "departments" {
            println!("{}", "Command accepted.".green());
            for key in &employees {
                println!("{}", key.0)
            }
        } else if command_words[0] == "help" {
            println!("{}", "Command accepted.".green());
            println!("{}", "Commands:\n- add 'employee' to 'department'\n- list 'department' (or 'all')\n- departments\n- help\n- quit\n".blue());
        } else if command_words[0] == "quit" {
            println!("{}", "Command accepted.".green());
            break;
        } else {
            println!("{}", "Command not valid.".red());
        }
    }
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝