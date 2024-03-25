// Introduction to the Copy Trait in Rust.

/*
The Copy trait in Rust is a special marker trait that can be implemented by types whose
values can be duplicated by simply copying bits (bitwise copy). Types that implement
the Copy trait can have their values copied to another variable without moving the original
value, allowing the original variable to be used even after the copy.

Simple scalar values like integers, floating-point numbers, and Booleans, as well as
tuples containing types that are also Copy, automatically implement this trait.

Strings in Rust, specifically the String type, cannot implement the Copy trait.
They can only be moved, or explicitly cloned if you want to keep the original value and create a new one.

The following examples demonstrate how the Copy trait works in Rust, showcasing the
differences between types that implement Copy and those that do not.

To run the code, type "cargo run copy" in the termnial.
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
    let y = x; // `x` implements the Copy trait, so it is not moved here.
    println!("x: {}, y: {}", x, y);
    // Demonstrates the basic behavior of the Copy trait with integer types.
    // When you run the code, the result ==> x: 5, y: 5
}

fn example2() {
    let x: (i32, f64) = (5, 4.3);
    let y = x; // Tuples containing types that implement Copy are themselves Copy.
    println!("x: {:?}, y: {:?}", x, y);
    // Showcases how tuples that contain Copy types are also Copy.
    // When you run the code, the result ==> x: (5, 4.3), y:(5, 4.3)
}

fn example3() {
    let x = true;
    let y = x; // Booleans also implement the Copy trait.
    println!("x: {}, y: {}", x, y);
    // Demonstrates the Copy trait with boolean values.
    // When you run the code, the result ==> x: true, y: true
}

fn example4() {
    let x: char = 'A';
    let y = x; // `char` type implements Copy.
    println!("x: {}, y: {}", x, y);
    // Highlights the Copy trait functionality with character types.
    // When you run the code, the result ==> x: A, y: A
}

fn example5() {
    let x = 2.71828f64;
    let y = x; // Floating-point types (f64 in this case) implement Copy.
    println!("x: {}, y: {}", x, y);
    // Demonstrates the Copy trait with floating-point numbers.
    // When you run the code, the result ==> x: 2.71828, y: 2.71828
}

// Define more nuanced examples showcasing Copy with arrays, function parameters,
// return values, and interactions with other Rust features like pattern matching.

fn example6() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1; // Arrays of Copy types are Copy if their size is known at compile time.
    println!("arr1: {:?}, arr2: {:?}", arr1, arr2);
    // Shows that arrays of types that are Copy, are themselves Copy.
    // When you run the code, the result ==> arr1: [1, 2, 3], arr2: [1, 2, 3]
}

fn example7() {
    let s = "Hello, world!"; // String literals are &'static str, which is Copy.
    let t = s; // `s` is not moved here, because it's a reference (&str) and is Copy.
    println!("s: {}, t: {}", s, t);
    // Explains how string slices (&str) are Copy.
    // When you run the code, the result ==> s: Hello, world!, t: Hello, world!
}

fn example8() {
    let num = Some(5);
    let num_copy = num; // Enums like Option<T> implement Copy if T is Copy.
    println!("num: {:?}, num_copy: {:?}", num, num_copy);
    // Demonstrates the Copy trait with enums (Option<T> in this case).
    // When you run the code, the result ==> num: Some(5), num_copy: Some(5)
}

fn example9() {
    fn takes_copy(x: i32) {
        println!("In takes_copy: x = {}", x);
        // When you run the code, the result ==> In takes_copy: x = 10
    }
    let x = 10;
    takes_copy(x);
    println!("After takes_copy: x = {}", x);
    // This example illustrates that passing a Copy type to a function doesn't move it.
    // When you run the code, the result ==> After takes_copy: x = 10
}

fn example10() {
    let flag = false;
    let flag_copy = clone_flag(flag); // Even though we're calling a function, flag is Copy, so it's not moved.
    println!("flag: {}, flag_copy: {}", flag, flag_copy);
    // Highlights that a Copy type remains after being passed to a function that returns its value.
    // When you run the code, the result ==> flag: false, flag_copy: false
}

fn clone_flag(f: bool) -> bool {
    f // Return the same value that was passed in.
}
