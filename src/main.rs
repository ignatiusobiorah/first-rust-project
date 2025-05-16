fn ini() {
    //string
    let word: &str = "this guy";
    println!("word: {}", word);

    //integer
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    //array
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("This is my favourite fruit: {:?}", fruits[0]);
    println!("I eat this frequently: {}", fruits[1]);
    println!("I only like this fruit as a juice: {}", fruits[2]);

    //mutubale string
    let mut name: String = String::from("Pope");
    println!("Name: {}", name);
    name.push_str("Francis");
    println!("Full Name: {}", name);

    //variable
    let mut a: &str = "Rust";
    println!("The value of a is: {}", a);
    a = "crab";
    println!("The value of a is now: {}", a);

    //variable shadowing
    let x: i32 = 5;
    {
        let x: &str = "Rust";
        println!("The inner scope x is: {}", x);
    }
    println!("The outer scope of x is: {}", x);
    let x: &str = "Crab";
    println!("The outer scope of x is now: {}", x);

    //tuple
    let tuple: (i32, f64, String) = (500, 6.4, "Rust".to_string());
    println!("The value of tup is: {:?}", tuple);

    //string
    let string: String = String::from("Rust");
    println!("The value of string is: {}", string);

    let string: String = String::from("Rust");
    println!("The value of string is: {}", string);

    let string1: String = "Rust1".to_string();
    println!("The value of string is: {}", string1);

    //while loop
    let mut number: i32 = 1;
    while number <= 10 {
        println!("2 x {} = {}", number, 2 * number);
        number += 1;
    }

    let mut x: i32 = 0;
    while x < 5 {
        println!("X is: {}", x);
        x += 1;
    }

    //using while loop with user input
    // use std::io;
    // let mut input = String::new();
    // println!("Enter text(type 'exit' to stop): ");
    // while input.trim() != "exit" {
    //     input.clear();
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     println!("You typed: {}", input.trim());
    // }
    // println!("Program terminated");

    //while loop with countdown
    let mut count: i32 = 10;
    while count >= 0 {
        println!("Countdown: {}", count);
        count -= 1;
    }
    println!("Blast off!");

    //Using break in while loop
    let mut count: i32 = 1;
    while count <= 10 {
        println!("Count: {}", count);

        if count == 5 {
            println!("Halfway there, exiting loop");
            break;
        }

        count += 1;
    }

    println!("Loop exited");

    //Using continue to skip an iteration
    for number in 1..=10 {
        //loop through 1 to 10
        if number % 2 == 0 {
            //check if number is even
            continue; //skip even numbers
        }
        println!("Odd number: {}", number);
    }

    //infinite while loop
    // let mut count: i32 = 0;
    // while true {
    //     println!("Count: {}", count);
    //     count += 1;

    //     if count == 6 {
    //         break;
    //     }
    // }

    // let mut count: i32 = 0; //initialize count to 0
    // while true { //infinite loop
    //     println!("Current Count: {}", count);
    //     count += 1; //increment count by 1

    //     if count >= 5 { //stop when the count reaches 5
    //         println!("Count reached 5, exiting loop");
    //         break;
    //         // count += 1; //increment count by 1
    //         println!("Loop exited. Final count {}", count);
    //     }

    // }

    //for loop
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    for fruit in fruits.iter() {
        println!("Fruit: {}", fruit);
    }

    //Simple function
    fn greet() {
        println!("Hello, Rust!");
    }
    greet();

    //create a function that finds out the average of several numbers and returns it
    fn average(numbers: &[i32]) -> f64 {
        let sum: i32 = numbers.iter().sum();
        let count: usize = numbers.len();
        sum as f64 / count as f64
    }

    let avg = average(&[1, 2, 3, 4, 5]);
    println!("The average is: {}", avg);

    //enums
    enum Light {
        Red,
        Green,
        Yellow,
    }

    fn action(light: Light) {
        match light {
            Light::Red => println!("Stop!"),
            Light::Green => println!("Go!"),
            Light::Yellow => println!("Caution!"),
        }
    }
    action(Light::Red);
    action(Light::Green);
    action(Light::Yellow);
}
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
fn main() {
    ini();
    // Reading a file
    let file: Result<File, std::io::Error> = File::open("PrudentLife Letter.pdf");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error);
                }
                _ => {
                    panic!("An error occurred: {}", error);
                }
            }
          
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(error) => panic!("Error reading line: {}", error),
        }
    }

    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);

}
