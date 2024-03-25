// Introduction to Tuples in Rust
// To run the code, type "cargo run tuples" in the terminal.

/*
Tuples in Rust are a versatile compound data type that allows you to combine together values of different types
into a single compound type. Each element in a tuple is called a field, and each field can be of a different type.
Tuples are fixed in size; once declared, the number of elements in a tuple cannot change, making them a flexible
tool for returning multiple values from a function or for temporarily grouping related values.

This section delves into the syntax for defining and using tuples, accessing their elements, and destructuring tuples
to extract their values. Tuples are particularly useful when you need to return multiple values from a function or
when you want to temporarily group together elements of different types without creating a more complex struct.

The examples provided will guide you through creating tuples, accessing their fields both by index and through
destructuring, and using tuples in functions. Understanding tuples is an important step in mastering Rust's type
system and its approach to organizing and manipulating diverse data in a safe, efficient manner.
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
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The tuple is: {:?}", tup);
    // This example demonstrates how to declare a tuple in Rust.
    // Tuples can contain elements of different types.
    // Here, tup is a tuple that contains an i32, an f64, and a u8.
    // When you run the code, the result ==>  The tuple is: (500, 6.4, 1)
}

fn example2() {
    let (x, y, z) = (1, 2.0, "three");
    println!("The value of y is: {}", y);
    // This example shows how to destructure a tuple,
    // assigning each value to a separate variable.
    // When you run the code, the result ==> The value of y is: 2
}

fn example3() {
    let tup = (10, 20, 30);
    let first = tup.0;
    let second = tup.1;
    println!(
        "The first value is: {}, and the second value is: {}",
        first, second
    );
    // Elements in a tuple can be accessed by their index using a period.
    // When you run the code, the result ==> The first value is: 10, and the second value is: 20
}

// Note: example4 intentionally demonstrates a non-compilable scenario
// and thus doesn't have an expected output in the comments.

fn example4() {
    // Intentionally incorrect code to demonstrate that tuples cannot be sliced
    // let tup = (1, 2, 3, 4, 5);
    // let slice = &tup[1..4]; // This line is not valid Rust code and will cause a compile error
    // println!("The tuple slice is: {:?}", slice);
    // Tuples in Rust cannot be sliced like arrays or vectors. This code, if uncommented, will not compile.
    // Rust's type system does not support slicing tuples because each element can be a different type.
    // When you attempt to compile this code, you will encounter a compiler error.
}

fn example5() {
    let tup = (500, 6.4);
    let (x, _) = tup;
    println!("The value of x is: {}", x);
    // This example shows how to use pattern matching to destructure a tuple
    // and ignore some of the values using an underscore.
    // When you run the code, the result ==> The value of x is: 500
}

fn example6() {
    let tup = (1, (2, 3), 4);
    let inner = tup.1 .1;
    println!("The inner value is: {}", inner);
    // Tuples can be nested within other tuples.
    // This example accesses an element from a nested tuple.
    // When you run the code, the result ==> The inner value is: 3
}

fn example7() {
    let x = (42,);
    println!("The single-element tuple is: {:?}", x);
    // A single-element tuple is defined with a trailing comma.
    // This distinguishes it from a value in parentheses.
    // When you run the code, the result ==> The single-element tuple is: (42,)
}

fn example8() {
    let tup = ("hello", 5, 'c');
    let (a, b, c) = tup;
    println!("The values are: {}, {}, {}", a, b, c);
    // Demonstrates destructuring a tuple with mixed types
    // and printing each value.
    // When you run the code, the result ==> The values are: hello, 5, c
}

fn example9() {
    let tup: (i32, f64) = (50, 5.5);
    let a = tup.0 + tup.1 as i32;
    println!("The sum of the elements is: {}", a);
    // This example shows how to perform operations on tuple elements.
    // Note: tup.1 is cast to i32 to allow addition.
    // When you run the code, the result ==> The sum of the elements is: 55
}

fn example10() {
    let empty = ();
    println!(
        "The unit type is represented by an empty tuple: {:?}",
        empty
    );
    // In Rust, the unit type `()` is a tuple with no elements.
    // It is used in places where an expression is needed, but no significant value is to be returned.
    // When you run the code, the result ==> The unit type is represented by an empty tuple: ()
}
