// Introduction to string literals.
// To run the code, type "cargo run literals" in the termnial.

/*
String literals in Rust are a convenient way to include static strings directly in your code.
They are immutable and stored in the program's binary, making them fast and efficient.
However, understanding their nuances, such as their type, lifetime, and how they differ from String objects, is crucial.
Let's explore string literals through examples, highlighting their characteristics and usage in Rust.
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
    let hello = "Hello, world!";
    println!("String literal: {}", hello);
    // Demonstrates a basic string literal, which is of type `&str`.
    // When you run the code, the result ==> String literal: Hello, world!
}

fn example2() {
    let s = "The quick brown fox jumps over the lazy dog";
    println!("String literal with length {}: {}", s.len(), s);
    // Shows getting the length of a string literal.
    // When you run the code, the result ==> String literal with length 43: The quick brown fox jumps over the lazy dog
}

fn example3() {
    let multiline = "This is a
multiline string literal
that spans several lines.";
    println!("{}", multiline);
    // Demonstrates a multiline string literal.
    // When you run the code, it prints the string across multiple lines as defined.
}

fn example4() {
    let with_escapes = "This string contains\na newline and a tab\t.";
    println!("{}", with_escapes);
    // Shows the use of escape characters in a string literal.
    // When you run the code, the newline and tab are interpreted as such.
}

fn example5() {
    let raw_string = r#"This is a raw string literal, where \n and \t are not interpreted."#;
    println!("{}", raw_string);
    // Demonstrates a raw string literal, where escape characters are not processed.
    // When you run the code, it prints exactly as it's defined, ignoring escape sequences.
}

fn example6() {
    let quotes_in_string = "She said, \"Hello, world!\"";
    println!("{}", quotes_in_string);
    // Shows how to include quotes in a string literal using escape characters.
    // When you run the code, the quotes are included in the output.
}

fn example7() {
    let long_raw_string = r###"
This is a raw string literal with "##" delimiters,
allowing for " and # characters without escapes.
"###;
    println!("{}", long_raw_string);
    // Demonstrates using custom delimiters in a raw string literal to include quotes and hashes.
    // When you run the code, it prints as defined, including all characters.
}

fn example8() {
    let concat_literals = "Hello,world!";
    println!("{}", concat_literals);
    // Concatenates two string literals at compile time.
    // When you run the code, the result ==> Hello, world!
}

fn example9() {
    const CONST_STR: &str = "I am a constant string literal.";
    println!("{}", CONST_STR);
    // Demonstrates defining a string literal as a constant.
    // When you run the code, the result ==> I am a constant string literal.
}

fn example10() {
    let emoji = "😊";
    println!("String literal with emoji: {}", emoji);
    // Shows that string literals can include Unicode characters, such as emojis.
    // When you run the code, the result ==> String literal with emoji: 😊
}
