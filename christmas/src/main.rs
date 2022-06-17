fn main() {
    let mut i: i8 = 1;
    let mut day = "first";
    let mut last_string = "A";

    clearscreen::clear().expect("failed to clear screen");
    println!("Twelve Days Of Christmas");

    while i <= 12 {
            println!("\nOn the {} day of Christmas\nMy true love sent to me", day);
        if i == 12 {
            println!("12 drummers drumming\nEleven pipers piping");
        }
        if i == 11 {
            println!("I sent 11 pipers piping");
            day = "12th";
        }
        if i >= 10 {
            println!("Ten lords a-leaping");
            if i == 10 {
                day = "11th";
            }
        }
        if i >= 9 {
            println!("Nine ladies dancing");
            if i == 9 {
                day = "tenth";
            }
        }
        if i >= 8 {
            println!("Eight maids a-milking");
            if i == 8 {
                day = "ninth";
            }
        }
        if i >= 7 {
            println!("Seven swans a-swimming");
            if i == 7 {
                day = "eighth";
            }
        }
        if i >= 6 {
            println!("Six geese a-laying");
            if i >= 7 {
                println!();
            }
            if i == 6 {
                day = "seventh";
            }
        }
        if i >= 5 {
            println!("Five golden rings (five golden rings)");
            if i == 5 {
                day = "sixth";
            }
        }
        if i >= 4 {
            println!("Four calling birds");
            if i == 4 {
                day = "fifth";
            }
        }
        if i >= 3 {
            println!("Three French hens");
            if i == 3 {
                day = "fourth";
            }
        }
        if i >= 2 {
            println!("Two turtle-doves");
            if i == 2 {
                day = "third";
                last_string = "And a";
            }
        }
        if i >= 1 {
            println!("{} partridge in a pear tree", last_string);
            if i == 12 {
                println!("{} partridge in a pear tree", last_string);
            }
            if i == 1 {
                day = "second";
            }
        }
    i+=1;
    }
}

//   ███████████████████╗
//  ██╔════════════════██╗
//  ╚█████╗        █████╔╝
//   ╚═══██╗       ╚═══██╗
//  ██████╔╝      ██████╔╝
//  ╚═════╝       ╚═════╝