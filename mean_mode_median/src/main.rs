//Given a list of integers, use a vector and return the mean (the average value),
//median (when sorted, the value in the middle position), and mode (the value that
//occurs most often; a hash map will be helpful here) of the list.

use std::{io, collections::HashMap};


fn main() {
    let mut list: Vec<i32> = Vec::new();
    println!("How many number does the list have?");
    let mut length = String::new();
    let mut counter: i32 = 1;

        io::stdin().read_line(&mut length).expect("Failed to read line!");
        let mut length: i32 = length.trim().parse().expect("Please type an integer number!");
    
    while length > 0 {
        println!("Number {}", counter);
        
        let mut number = String::new(); 
        io::stdin().read_line(&mut number).expect("Failed to read line!");
        let number: i32 = number.trim().parse().expect("Please type an integer number!");
        
        list.push(number);
        
        length -= 1;
        counter += 1;
    }

    let mut values = HashMap::new();
    values.insert(String::from("Mean"), mean(&list));
    values.insert(String::from("Median"), median(&list).0);
    if median(&list).2 == true {
        values.insert(String::from("Median 2"), median(&list).1);
    }
    values.insert(String::from("Mode"), mode(&list));

    mode(&list);

    for (key, value) in &values {
        println!("{}: {}", key, value);
    }
}

fn mean(list: &Vec<i32>) -> f64 {
    let mut length = list.len() as i32;
    length -= 1;
    let mut mean: i32 = 0;
    while length >= 0 {
        mean += &list[length as usize];
        length -= 1;
    }
    let mean: f64 = mean as f64;

    mean / list.len() as f64
}

fn median(list: &Vec<i32>) -> (f64, f64, bool) {
    
    let mut ordered_list: Vec<i32> = list.clone();
    let mut i = 0;
    while i <= ordered_list.len()-1
    {  
        let mut j = i+1;
        while j < ordered_list.len()
        {  
            if ordered_list[i] > ordered_list[j]  
            {  
                let temp = ordered_list[i];  
                ordered_list[i] = ordered_list[j];  
                ordered_list[j] = temp;  
            }  
            j+=1;
        }  
        i+=1;
    }  

    if ordered_list.len()%2 == 0 {
        return (ordered_list[ordered_list.len()/2-1] as f64, ordered_list[ordered_list.len()/2] as f64, true)
    } else {
        return (ordered_list[ordered_list.len()/2] as f64, ordered_list[ordered_list.len()/2] as f64, false)
    }
}

fn mode(list: &Vec<i32>) -> f64 {
    let mut numbers_counter = HashMap::new();
        for number in list {
            let count = numbers_counter.entry(number).or_insert(0);
            *count += 1;
        }
    
    let mut max_value = &i32::MIN;
    let mut max_value_key = &0;
    for (key, value) in &numbers_counter {
        if value > max_value {
            max_value_key = key;
            max_value = value;
        }
    }

    *max_value_key as f64
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝