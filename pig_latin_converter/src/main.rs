/*Convert strings to pig latin. The first consonant of each word is moved
to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple”
becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!*/

use std::io;

fn main() {
    println!("Type a word or a phrase to covernt in Pig Latin");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    
    for word in converter(&input) {
        println!("{}", word);
    }
}

fn converter (user_input: &String) -> Vec<String>{
    let mut output = Vec::new(); 
    for word in user_input.split_whitespace() {
        let mut prefix = '\0';
        let mut converted_word = String::from("\0");
        let mut first_letter = true;
        for c in word.chars() {
            if first_letter {
                prefix = c;
                first_letter = false;
            } else {
                converted_word = format!("{}{}",converted_word, c);
            }  
        }
        match prefix {
            'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => output.push(format!("{}-hay", word)),
            _ => output.push(format!("{}-{}ay", converted_word, prefix))
        }
    }
    output
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝