// Introduction to Variables and Mutability in Rust
// To run the code, type "cargo run variables" in the termnial.

/*
Rust provides a strong system of variables, types, and mutability that is central to the language's promise of safety and performance.
In Rust, variables are immutable by default, which encourages a coding style that leads to safer code by making it explicit when data
is intended to be changed. This section explores how to declare variables, the significance of immutability, and how to explicitly
opt-in to mutability when necessary.

Understanding variables and mutability in Rust is crucial for writing idiomatic Rust code. It involves learning about the `let`
keyword for variable declaration, the `mut` keyword to make variables mutable, and the constants declared using `const`, which
are always immutable and must have their type annotated.

The examples provided will cover the basics of declaring variables, the rules of mutability, shadowing (redeclaring variables
to change their type or value), and using constants. By mastering these concepts, you will be well on your way to developing
safe and efficient Rust programs that leverage Rust's powerful type system and memory safety guarantees.
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
    let x = 5;
    println!("The value of x is: {}", x);
    // This example demonstrates how to declare an immutable variable in Rust.
    // By default, variables are immutable, meaning once a value is assigned, it cannot be changed.
    // When you run the code, the result ==> The value of x is: 5
}

fn example2() {
    let x = 5;
    // Uncommenting the next line will cause a compile-time error because x is immutable
    // x = 6;
    println!("The value of x is: {}", x);
    // This illustrates immutability in Rust, enforcing safety and concurrency benefits.
    // When you run the code, the result ==> The value of x is: 5
}

fn example3() {
    let mut x = 5; // mutable, value can change.
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is now: {}", x);
    // The `mut` keyword allows the variable `x` to be mutable,
    // meaning its value can be changed after it's initially set.
    // When you run the code, the results ==> The value of x is: 5
    //                                       The value of x is now: 6
}

fn example4() {
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
    // Constants are always immutable and must be typed.
    // They can be declared in any scope and have a fixed value for the duration
    // of a program's execution.
    // When you run the code, the result ==> The maximum points are: 100000
}

fn example5() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is now: {}", x);
    let x = x * 2;
    println!("The value of x is now: {}", x);
    // Shadowing allows us to reuse the variable name `x` for a new value.
    // It’s different from marking a variable as `mut`, because it effectively creates a new variable each time.
    // When you run the code, the results ==> The value of x is: 5
    //                                       The value of x is now: 6
    //                                       The value of x is now: 12
}

fn example6() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
    // Shadowing can also be used to change the type of a variable from one value to another.
    // Here, `spaces` changes from a string to an integer.
    // When you run the code, the result ==> The number of spaces is: 3
}

fn example7() {
    let x: i32 = -123;
    println!("x is: {}", x);
    // Rust supports various integer types. This example uses `i32`, a 32-bit signed integer.
    // Type annotations are optional unless required by the context.
    // When you run the code, the result ==> x is: -123
}

fn example8() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x is: {}, y is: {}", x, y);
    // Rust has two floating-point types, `f64` and `f32`. By default, `f64` is used,
    // as it’s roughly the same speed as `f32` but is capable of more precision.
    // When you run the code, the results ==> x is: 2.0, y is: 3.0
}

fn example9() {
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t is: {}, f is: {}", t, f);
    // Booleans in Rust have two possible values: `true` and `false`.
    // Rust’s boolean type is specified using `bool`.
    // When you run the code, the results ==> t is: true, f is: false
}

fn example10() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!(
        "c is: {}, z is: {}, heart_eyed_cat is: {}",
        c, z, heart_eyed_cat
    );
    // The `char` type is the most primitive alphabetic type in Rust,
    // representing a single Unicode scalar value. It is specified with single quotes.
    // When you run the code, the results ==> c is: z, z is: ℤ, heart_eyed_cat is: 😻
}
