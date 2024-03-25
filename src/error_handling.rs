/*
Introduction to Error Handling in Rust:

Error handling in Rust is a vital aspect of writing reliable, robust software.
Rust encourages a paradigm where errors are expected outcomes that can be handled gracefully.
This approach to error handling is manifest in two primary ways: the Result and Option enums.

The Result<T, E> type is used when an operation could logically fail, where T represents a success
type and E an error type. The Option<T> type encodes the scenario where a value could be
None (absent) or Some(value). Rust’s pattern matching capabilities shine here,
allowing for expressive and comprehensive handling of these cases.

This section will cover the basics of using Result and Option, how to propagate errors,
and how to create custom error types. These practices empower you to write clean, maintainable Rust
code that effectively manages and responds to error conditions.
*/
// Demonstrates error handling in Rust.

pub fn run_examples() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    example8();
    example9();
    example10();
}

fn example1() {
    let result: Result<i32, &str> = Ok(200);
    match result {
        Ok(code) => println!("Code: {}", code),
        Err(error) => println!("Error: {}", error),
    }
    // Demonstrates basic usage of the Result type for error handling.
    // When you run the code, the result ==> Code: 200
}

fn example2() {
    let result: Result<i32, &str> = Err("Error occurred");
    match result {
        Ok(code) => println!("Code: {}", code),
        Err(error) => println!("Error: {}", error),
    }
    // Shows handling an error case with Result.
    // When you run the code, the result ==> Error: Error occurred
}

fn example3() {
    let some_option = Some(5);
    let absent_option: Option<i32> = None;

    println!("Some option: {:?}", some_option.unwrap_or(0));
    println!("Absent option: {:?}", absent_option.unwrap_or(0));
    // Demonstrates using Option for values that might be absent, and unwrapping them with a default.
    // When you run the code, the result ==> Some option: 5
    //                                      Absent option: 0
}

fn example4() {
    let result = divide(10.0, 2.0);
    match result {
        Ok(quotient) => println!("Quotient: {}", quotient),
        Err(e) => println!("Error: {}", e),
    }
    // Demonstrates propagating errors with the `?` operator in functions that return Result.
    // When you run the code, the result ==> Quotient: 5.0
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("Cannot divide by zero.")
    } else {
        Ok(numerator / denominator)
    }
}

fn example5() {
    let file_path = "non_existent_file.txt";
    match read_file(file_path) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
    // Demonstrates handling errors from file operations with pattern matching.
    // When you run the code, it will likely result in an error message since the file does not exist.
}

use std::fs;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::File::open(path).and_then(|mut file| {
        let mut contents = String::new();
        file.read_to_string(&mut contents).map(|_| contents)
    })
}

fn example6() {
    let result = some_computation();
    if let Err(e) = result {
        println!("Error: {}", e);
    }
    // Introduces using `if let` for error handling when you're only interested in the error case.
    // When you run the code, the result depends on the computation's outcome.
}

fn some_computation() -> Result<(), &'static str> {
    Err("An error occurred")
}

fn example7() {
    let some_value: Option<i32> = Some(10);
    println!(
        "Value squared: {}",
        some_value.map_or_else(|| -1, |v| v.pow(2))
    );
    // Showcases using Option's `map_or_else` method for handling potential None values with a default.
    // When you run the code, the result ==> Value squared: 100
}

fn example8() {
    let vec = vec![1, 2, 3, 4, 5];
    let index = 2;

    match vec.get(index) {
        Some(&number) => println!("Value at index {}: {}", index, number),
        None => println!("No element at index {}!", index),
    }
    // Demonstrates safely accessing vector elements using Option and pattern matching.
    // When you run the code, the result ==> Value at index 2: 3
}

fn example9() {
    let fruits = vec!["apple", "banana", "cherry"];
    for fruit in fruits
        .iter()
        .filter_map(|&fruit| if fruit.len() > 5 { Some(fruit) } else { None })
    {
        println!("Long fruit: {}", fruit);
    }
    // Using `filter_map` to filter and transform items in an iterator with Option.
    // When you run the code, the result will list fruits with more than 5 characters.
}

fn example10() {
    let password = "supersecret";
    match check_password(password) {
        Ok(()) => println!("Access granted."),
        Err(e) => println!("Access denied: {}", e),
    }
    // A simple example of using custom error types for validation.
    // When you run the code, the result depends on the password check's outcome.
}

fn check_password(password: &str) -> Result<(), &'static str> {
    if password.len() >= 8 {
        Ok(())
    } else {
        Err("Password too short!")
    }
}
