// Introduction to Function.
// To run the code, type "cargo run functions" in the termnial.

/*
Functions are central to Rust programming, allowing you to organize code into reusable blocks.
They can accept parameters, return values, and are defined using the fn keyword.
Rust's type system is also reflected in function signatures, where you must specify the type of each parameter and the return type of the function.
This section will explore various aspects of functions in Rust, from basic definitions to more advanced features like closures and higher-order functions.
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
    println!("Sum of 5 and 3 is: {}", add(5, 3));
    // Demonstrates a simple function that adds two numbers.
    // When you run the code, the result ==> 4 is even
}

fn add(x: i32, y: i32) -> i32 {
    x + y // Rust implicitly returns the last expression in the function body.
}

fn example2() {
    let squared = square(4);
    println!("4 squared is: {}", squared);
    // Shows a function that takes an integer and returns its square.
    // When you run the code, the result ==> 4 squared is: 16
}

fn square(x: i32) -> i32 {
    x * x
}

fn example3() {
    let no_return = no_value();
    println!("Function that returns unit: {:?}", no_return);
    // Illustrates a function that returns the unit type `()`.
    // When you run the code, the result ==> Function that returns unit: ()
}

fn no_value() {
    // This function doesn't explicitly return a value, so it returns `()`.
}

fn example4() {
    if is_even(4) {
        println!("4 is even");
    } else {
        println!("4 is odd");
    }
    // Demonstrates a function that checks if a number is even.
    // When you run the code, the result ==> 4 is even.
}

fn is_even(x: i32) -> bool {
    x % 2 == 0
}

fn example5() {
    let factorial_of_5 = factorial(5);
    println!("Factorial of 5 is: {}", factorial_of_5);
    // Shows a function that calculates the factorial of a number using recursion.
    // When you run the code, the result ==> Factorial of 5 is: 120
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn example6() {
    let print = || println!("This is a closure without parameters.");
    print();
    // Demonstrates defining and using a simple closure.
    // When you run the code, the result ==> This is a closure without parameters.
}

fn example7() {
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("5 plus 1 is: {}", add_one(5));
    // Shows a closure that takes a parameter and returns a value.
    // When you run the code, the result ==> 5 plus 1 is: 6
}

fn example8() {
    let x = 10;
    let add_x = |y: i32| -> i32 { x + y };
    println!("10 plus 5 is: {}", add_x(5));
    // Illustrates how closures can capture variables from their surrounding environment.
    // When you run the code, the result ==> 10 plus 5 is: 15.
}

fn example9() {
    let double = |x| 2 * x;
    apply_function(5, double);
    // Demonstrates a higher-order function that takes a function as a parameter.
    // Note: Output is shown in the `apply_function` implementation.
}

fn apply_function<F>(x: i32, f: F)
where
    F: Fn(i32) -> i32,
{
    let result = f(x);
    println!("Result of applying function: {}", result);
    // When you run the code, the result ==> Result of applying function: 10
}

fn example10() {
    let numbers = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<_> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers from the list: {:?}", even_numbers);
    // When you run the code, the result ==> Result of applying function: 10
    // Shows using a closure with `filter` to operate on a collection.
}
