use std::io;
use std::io::Read;

fn main() {
    loop {
        println!("Select an option: \n1 - Farhenheit to Celsius\n2 - Celsius to Farhenheit\n0 - Exit");
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line!");
        let option: i8 = option.trim().parse().expect("Please type 1, 2 or 0!");
        
        if option == 0 {
            println!("Exiting...");
            break;
        }

        println!("Type the degrees to convert");
        let mut degrees = String::new();
        io::stdin().read_line(&mut degrees).expect("Failed to read line!");
        let degrees: f64 = degrees.trim().parse().expect("Please type a floating number");

        if option == 2 {
            println!("{}°C = {}°F", degrees, celsius_to_farhenheit(degrees)); 
        } else if option == 1 {
            println!("{}°F = {}°C", degrees, farhenheit_to_celsius(degrees)); 
        } else {
            println!("Choose a valid option! try again.");
        }

        println!("Press ENTER to continue...");
        let buffer = &mut [0u8];
        std::io::stdin().read_exact(buffer).unwrap();
        print!("{}[2J", 27 as char);
    }
}

fn farhenheit_to_celsius (degrees: f64) -> f64{
    (degrees - 32.0) * 1.8
}

fn celsius_to_farhenheit (degrees: f64) -> f64{
    (degrees * 1.8) + 32.0
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝