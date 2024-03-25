// Introduction to References and Borrowing in Rust

/*
Rust's ownership system is a core part of its commitment to memory safety and concurrency without data races.
References and borrowing are mechanisms that allow functions to access data without taking ownership of it,
enabling safe, concurrent access to data across different parts of a program. This section explores how
references (`&T`) for shared access and mutable references (`&mut T`) for exclusive, mutable access work,
alongside Rust's borrowing rules that enforce safety at compile time.
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
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // Demonstrates borrowing a String as a reference to calculate its length without taking ownership
    // When you run the code, the result ==> The length of 'hello' is 5.
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s is a reference to a String, allowing the function to use the String without owning it.
}

fn example2() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);
    // Showcases mutable borrowing, allowing a function to modify a String without taking ownership.
    // When you run the code, the result ==> Changed string: hello, world
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn example3() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // Demonstrates multiple immutable references to the same value are allowed.
    // When you run the code, the result ==> hello and hello
}

fn example4() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // When you run the code, the result ==> hello and hello
    // Even though s is mutable, it's borrowed as immutable, so multiple references are allowed.
}

fn example5() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);
    // Demonstrates that you can have a single mutable reference to a value in a particular scope.
    // When you run the code, the result ==> hello
}

fn example6() {
    let s = String::from("hello, world");
    let word = first_word(&s);
    println!("The first word is: '{}'", word);
    // Demonstrates Rust's prevention of dangling references.
    // When you run the code, the result ==> The first word is: 'hello'
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn example7() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
    // Demonstrates borrowing part of a String as a slice.
    // When you run the code, the result ==> hello and hello
    //
}

fn example8() {
    let mut s = String::from("hello, world");
    clear_string(&mut s);
    println!("Cleared string: '{}'", s);
    // When you run the code, the result ==> Cleared string: ''
}

fn clear_string(s: &mut String) {
    s.clear();
}

fn example9() {
    let mut count = 0;
    let count_ref = &mut count;
    *count_ref += 1;
    println!("Count: {}", count);
    // When you run the code, the result ==> Count: 1
}

fn example10() {
    let s = String::from("hello, world");
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("Sliced: '{}' and '{}'", hello, world);
    // When you run the code, the result ==> Sliced: 'hello' and 'world'
}
