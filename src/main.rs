fn main() {
    println!("Hello, Rust!");

    string
    let word: &str = "this guy";
    println!("word: {}", word);

    integer
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);

    array
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("This is my favourite fruit: {:?}", fruits[0]);
    println!("I eat this frequently: {}", fruits[1]);
    println!("I only like this fruit as a juice: {}", fruits[2]);

    mutubale string
    let mut name: String = String::from("Pope");
    println!("Name: {}", name);
    name.push_str("Francis");
    println!("Full Name: {}", name);

    variable
    let a: &str ="Rust";
    println!("The value of a is: {}", a);
    a = "crab";
    println!("The value of a is now: {}", a);

    variable shadowing
    let x: i32 = 5;
    {
        let x: &str = "Rust";
        println!("The inner scope x is: {}", x);
    }
    println!("The outer scope of x is: {}", x);
    let x: &str = "Crab";
    println!("The outer scope of x is now: {}", x);

    tuple
    let tuple: (i32, f64, String) = (500, 6.4, "Rust".to_string());
    println!("The value of tup is: {:?}", tuple);

    string
    let string: String = String::from("Rust");
    println!("The value of string is: {}", string);

    let string: String = String::from("Rust");
    println!("The value of string is: {}", string);

    let string1: String = "Rust1".to_string();
    println!("The value of string is: {}", string1);
}