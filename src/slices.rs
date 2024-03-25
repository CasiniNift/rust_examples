// Introduction to Slices in Rust
// To run the code, type "cargo run slices" in the terminal.

/*
Slices in Rust provide a way to reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice.
Slices are especially useful because they allow safe, efficient access to a portion of an array without copying its contents.
This makes slices a powerful tool for efficient data processing and manipulation.

In Rust, slices can be used with various types of collections, including arrays and strings. This section will explore how
to declare and use slices, how to work with string slices (str), and how to utilize slices to achieve more efficient and
expressive code. Understanding slices is crucial for Rust programming, as they play a significant role in memory safety
and data access patterns.

The examples provided will demonstrate practical uses of slices, including iterating over elements, accessing parts of strings,
and modifying data in-place. By learning about slices, you'll gain insight into Rust's approach to safe, efficient memory
management and how it differs from traditional pointer arithmetic found in languages like C or C++.
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
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("Slice of arr: {:?}", slice);
    // Demonstrates creating a slice from an array. Slices provide a view into a subset of the array elements.
    // You are choosing the elements from arr[1] to arr[3] and not including arr[4]
    // When you run the code, the result ==> Slice of arr: [2, 3, 4]
}

fn example2() {
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[1..4];
    println!("Slice of vec: {:?}", slice);
    // Similarly, this example shows how to create a slice from a vector.
    // You are choosing the elements from vec[1] to vec[3] and not including vec[4].
    // When you run the code, the result ==> Slice of vec: [2, 3, 4].
}

fn example3() {
    let s = String::from("Hello, world!");
    let slice = &s[0..5];
    println!("Slice of string: {}", slice);
    // Demonstrates creating a string slice. String slices allow you to reference a portion of a String.
    // In the string slice, you are incuding s[0] to s[4] but not including s[5]
    // When you run the code, the result ==> Slice of string: Hello.
}

fn example4() {
    let s = "Hello, world!";
    let slice = &s[7..12];
    println!("Another slice of string: {}", slice);
    // Shows creating a slice from a string literal directly.
    // In the string slice, you are incuding s[7] to s[11] but not including s[12]
    // When you run the code, the result ==> Another slice of string: world
}

fn example5() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..]; // Equivalent to &arr[0..arr.len()]
    println!("Full slice of arr: {:?}", slice);
    // Creating a slice that includes all elements of the array.
    // When you run the code, the result ==> Full slice of arr: [1, 2, 3, 4, 5]
}

fn example6() {
    let s = "Hello, world!";
    let word = first_word(&s);
    println!("First word: {}", word);
    // Using a slice to get the first word of a string.
    // When you run the code, the result ==> First word: Hello
}

fn example7() {
    let arr = [10, 20, 30, 40, 50];
    let middle_slice = &arr[1..4];
    for i in middle_slice.iter() {
        println!("Value: {}", i);
    }
    // Iterating over a slice of an array.
    // When you run the code, it prints each value in the slice on a new line.
    // The values printed are 20,30,40
}

fn example8() {
    let vec = vec![100, 32, 57];
    let slice = &vec[..2];
    println!("Partial slice of vec: {:?}", slice);
    // Demonstrates creating a partial slice from the beginning of a vector.
    // You are choosing the values from vec[0] to vec[1] snd not including vec[2].
    // When you run the code, the result ==> Partial slice of vec: [100, 32].
}

fn example9() {
    let s = "slice of a string";
    let slice = &s[6..];
    println!("Slice from the middle of a string: {}", slice);
    // Creating a slice from the middle of a string to its end.
    // You are choosing the values from s[6] to the end.
    // When you run the code, the result ==> Slice from the middle of a string: of a string
}

fn example10() {
    let mut arr = [1, 2, 3, 4, 5];
    let slice_mut = &mut arr[1..4];
    slice_mut[0] = 10;
    println!("Modified array through slice: {:?}", arr);
    // Modifying an array through a mutable slice.
    // When you run the code, the result ==> Modified array through slice: [1, 10, 3, 4, 5]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
