// Introduction to Arrays in Rust.
// To run the code, type "cargo run arrays" in the termnial.

/*
In Rust, an array is a collection of elements of the same type, stored in contiguous memory locations.
Arrays are useful when you want to allocate a fixed number of elements. The size of an array is known at compile time,
making arrays a stack-allocated data structure. This section covers various aspects of arrays, including declaration,
initialization, accessing elements, and iterating over them. Arrays in Rust offer performance benefits due to their
fixed size and the fact that they are stored on the stack, but they also impose limitations on flexibility compared
to other collection types like vectors.

The examples provided here demonstrate the basics of working with arrays, including how to declare them,
how to access and modify elements, and how to use arrays in functions. Understanding arrays is fundamental
to grasping Rust's approach to data management and memory safety, as they are one of the simplest compound
types in the language.
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
    let nums = [1, 2, 3, 4, 5];
    println!("The array is: {:?}", nums);
    // Arrays are declared using square brackets.
    // This example declares an array of integers.
    // When you run the code, the result ==> The array is: [1, 2, 3, 4, 5]
}

fn example2() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array with explicit type and size is: {:?}", nums);
    // Specifying the type and size of the array explicitly.
    // When you run the code, the result ==> The array with explicit type and size is: [1, 2, 3, 4, 5]
}

fn example3() {
    let nums = [3; 5]; // the number 3 to be repeated 5 times.
    println!("The array with same elements is: {:?}", nums);
    // Initializes an array of 5 elements, all set to the value 3.
    // When you run the code, the result ==> The array with same elements is: [3, 3, 3, 3, 3]
}

fn example4() {
    let nums = [1, 2, 3, 4, 5];
    let first = nums[0];
    println!("The first element is: {}", first);
    // Accessing array elements by index.
    // When you run the code, the result ==> The first element is: 1
}

fn example5() {
    let nums = [1, 2, 3, 4, 5];
    let len = nums.len();
    println!("The length of the array is: {}", len);
    // The len() method returns the number of elements in the array.
    // When you run the code, the result ==> The length of the array is: 5
}

fn example6() {
    let mut nums = [1, 2, 3, 4, 5];
    nums[2] = 99;
    println!("The modified array is: {:?}", nums);
    // Modifying an array element at a specific index.
    // When you run the code, the result ==> The modified array is: [1, 2, 99, 4, 5]
}

fn example7() {
    let nums = [1, 2, 3, 4, 5];
    for num in nums.iter() {
        // .iter prients every value on it's own line.
        println!("The number is: {}", num);
    }
    // Iterating over the elements of an array using iter().
    // When you run the code, each element will be printed on its own line.
}

fn example8() {
    let nums: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("The 2D array is: {:?}", nums);
    // Declaring a two-dimensional array.
    // Here, you are asking to do 2 arrays, each with 3 values.
    // When you run the code, the result ==> The 2D array is: [[1, 2, 3], [4, 5, 6]]
}

fn example9() {
    let nums = [1, 2, 3, 4, 5];
    let slice = &nums[1..4];
    println!("The slice of the array is: {:?}", slice);
    // Slicing an array to get a subset of its elements.
    // When you run the code, the result ==> The slice of the array is: [2, 3, 4]
}

fn example10() {
    let nums = [1, 2, 3, 4, 5];
    match nums.get(5) {
        Some(x) => println!("The element is: {}", x),
        None => println!("There is no element at this index."),
    }
    // Safely accessing an element of an array using the get() method.
    // This prevents runtime errors if the index is out of bounds.
    // When you run the code, the result ==> There is no element at this index.
}
