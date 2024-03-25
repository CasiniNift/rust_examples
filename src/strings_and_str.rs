// Introduction to strings and &str
// To run the code, type "cargo run strings" in the termnial.

/*
Strings in Rust come in two main types: String and &str.
The String type is a growable, mutable, owned collection of characters.
The &str type, often referred to as a "string slice", is a view into a string and is typically immutable.
Understanding the distinction and how to work with both types is crucial in Rust. Let's explore these concepts with examples.
*/

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
    let s: String = String::from("Hello, world!");
    println!("String: {}", s);
    // Demonstrates creating a String from a string literal.
    // When you run the code, the result ==> String: Hello, world!
}

fn example2() {
    let mut s = "Hello".to_string();
    s.push_str(", world!");
    println!("Mutable String: {}", s);
    // Shows how to append a string slice to a String.
    // "Hello".to_string() means that "Hello" will be added somewhere latter.
    // s.push_str mans that you will add the privous part.
    // When you run the code, the result ==> Mutable String: Hello, world!
}

fn example3() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: s1 has been moved here and can no longer be used
    println!("Concatenated String: {}", s3);
    // Demonstrates string concatenation using the `+` operator.
    // When you run the code, the result ==> Concatenated String: Hello, world!
}

fn example4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // `format!` does not take ownership
    println!("Formatted String: {}", s);
    // Using the `format!` macro for string concatenation without taking ownership.
    // When you run the code, the result ==> Formatted String: tic-tac-toe
}

fn example5() {
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("Slice of String: {}", slice);
    // Demonstrates creating a string slice from a String.
    // You include only &s[0] to &s[1].
    // When you run the code, the result ==> Slice of String: he
}

fn example6() {
    let s = "Hello, world!";
    println!("String slice: {}", s);
    // Shows that string literals are of type `&str`, essentially string slices.
    // When you run the code, the result ==> String slice: Hello, world!
}

fn example7() {
    let mut s = String::new();
    s.push('H');
    s.push_str("ello");
    println!("Build a String: {}", s);
    // Demonstrates building a String one piece at a time.
    // When you run the code, the result ==> Build a String: Hello
}

fn example8() {
    let s = String::from("hello world");
    let words: Vec<&str> = s.split_whitespace().collect();
    println!("Words from String: {:?}", words);
    // Splitting a String into a vector of &str slices based on whitespace.
    // When you run the code, the result ==> Words from String: ["hello", "world"]
}

fn example9() {
    let s = "hello";
    let s = s.to_string(); // Converts a `&str` to a `String`
    println!("&str to String: {}", s);
    // Demonstrates converting a `&str` to a `String`.
    // When you run the code, the result ==> &str to String: hello
}

fn example10() {
    let s = String::from("hello");
    let s = &s; // Now s is a `&String`, which can be coerced to `&str`
    println!("String to &str: {}", s);
    // Shows how a `&String` reference can be used where `&str` is expected.
    // When you run the code, the result ==> String to &str: hello
}
