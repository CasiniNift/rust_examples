// Introduction to Move Semantics in Rust

// In Rust, variables own their data. When data is assigned from one variable to another,
// or when it's passed to a function, the ownership of that data is transferred, or "moved".
// This concept, known as "move semantics", is a key part of Rust's approach to memory safety
// and management. After a move, the original variable can no longer be used, preventing
// accidental duplication of data or invalid references.

// The following examples demonstrate various scenarios involving move semantics,
// helping you understand how Rust's ownership model handles data movement.

// To run the code, type "cargo run move" in the termnial.

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
    let x = String::from("hello");
    let y = x;
    // println!("x: {}", x); // This line would cause a compile error because the ownership of the value in x has been moved to y.
    println!("y: {}", y);
    // When you run the code, the result ==> y: hello
}

fn example2() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // Since integers are simple values with a known, fixed size, and they implement the Copy trait,
    // both x and y can be used after the assignment. This is not a move but a copy.
    // When you run the code, the result ==> x: 5, y: 5
}

fn example3() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s: {}", s); // s can't be used here as its ownership has been moved to the function.
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
} // When you run the code, the result ==> some_string: hello

fn example4() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);
    // When you run the code, the result ==> s1: yours
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3: {}", s3);
    // s2 is not valid here but s3 is because ownership of s2's data was moved into the function
    // and then back to s3.
    // When you run the code, the result ==> s3: hello
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// Continue with more examples illustrating different aspects of move semantics
// Examples 5 through 10 would dive deeper into functions, return values, and more complex data structures,
// demonstrating the versatility and safety of move semantics in Rust.

fn example5() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    // println!("v1: {:?}", v1); // Error: use of moved value: `v1`
    println!("v2: {:?}", v2);
    // This demonstrates move semantics with vectors, a non-Copy, heap-allocated data structure.
    // When you run the code, the result ==> v2: [1, 2, 3]
}

fn example6() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let person2 = person;
    // println!("person: {:?}", person); // This line will cause a compile error because `person` has been moved to `person2`.
    println!("person2: name = {}, age = {}", person2.name, person2.age);
    // Demonstrates move semantics with custom data types (structs).
    // When you run the code, the result ==> person2: name = Alice, age = 30
}

struct Person {
    name: String,
    age: i32,
}

fn example7() {
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[1..3];
    println!("slice: {:?}", slice);
    // This example shows that slicing a vector does not take ownership of the vector.
    // It merely borrows a part of it, demonstrating that not all operations move or copy data.
    // When you run the code, the result ==> slice: [2, 3]
}

fn example8() {
    let name = String::from("Bob");
    let _ = move_to_function(name);
    // println!("name: {}", name); // Error: use of moved value: `name`. `name`'s ownership has been moved into the function.
    // This highlights how passing a value to a function moves its ownership, and the original variable cannot be used afterward.
}

fn move_to_function(s: String) -> String {
    println!("Function owns the string: {}", s);
    s // Return the ownership back to the caller
      // When you run the code, the result ==> Function owns the string: Bob
}

fn example9() {
    let color = Color::Rgb(0, 127, 255);
    match color {
        Color::Rgb(r, g, b) => println!("RGB Color - Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::Hsv(h, s, v) => println!("HSV Color - Hue: {}, Saturation: {}, Value: {}", h, s, v),
    }
    // println!("Original color: {:?}", color); // This will cause a compile error because `color` has been moved in the match.
    // Demonstrates move semantics in pattern matching with enums. Enum values are moved when deconstructed in a match.
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn example10() {
    let book = Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
    );
    present_book(book);
    // println!("book title: {}", book.title); // Error: use of moved value: `book`. `book` has been moved into the `present_book` function.
    // Showcases how user-defined function arguments participate in move semantics, transferring ownership.
}

struct Book {
    title: String,
    authors: String,
}

impl Book {
    fn new(title: &str, authors: &str) -> Book {
        Book {
            title: title.to_string(),
            authors: authors.to_string(),
        }
    }
}

fn present_book(book: Book) {
    println!("Presenting book: {} by {}", book.title, book.authors);
    // When you run the code, the result ==> Presenting book: The Rust Programming Language by Steve Klabnik and Carol Nichols
}
