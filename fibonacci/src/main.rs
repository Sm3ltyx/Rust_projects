use std::io;

fn main() {
    println!("Type 2 integer number.");
    let mut n1 = String::new();
    println!("Number 1");
    io::stdin().read_line(&mut n1).expect("Failed to read line!");
    let n1: u64 = n1.trim().parse().expect("Please type an integer number!");
    let mut n2 = String::new();
    println!("Number 2");
    io::stdin().read_line(&mut n2).expect("Failed to read line!");
    let n2: u64 = n2.trim().parse().expect("Please type an integer number!");

    fibonacci(n1, n2);
}

fn fibonacci(mut n1: u64, mut n2: u64){
    println!("Fibonacci\n{}\n{}", n1, n2);    
    loop {
        n1 = n1+n2;
        println!("{}", n1);
        n2 = n1+n2;
        println!("{}", n2);
        //Stop by overflow
    }
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝