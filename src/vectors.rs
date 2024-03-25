// To run the code, type "cargo run vectors" in the termnial.
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
    let v: Vec<i32> = Vec::new();
    println!("A new vector: {:?}", v);
    // Declares a new empty vector `v` of type `i32`.
    // Vectors are dynamic arrays that can grow and shrink in size.
    // When you run the code, the result ==> A new vector: []
}

fn example2() {
    let v = vec![1, 2, 3, 4, 5];
    println!("A vector created using the vec! macro: {:?}", v);
    // Uses the vec! macro to create and initialize a vector.
    // This is a shorthand for creating vectors with initial values.
    // When you run the code, the result ==> A vector created using the vec! macro: [1, 2, 3, 4, 5]
}

fn example3() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector after pushing elements: {:?}", v);
    // Demonstrates adding elements to a vector using the `push` method.
    // Note that the vector has to be mutable to change its content.
    // When you run the code, the result ==> Vector after pushing elements: [5, 6, 7, 8]
}

fn example4() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // Accesses an element of a vector by indexing.
    // This will panic at runtime if the index is out of bounds.
    // When you run the code, the result ==> The third element is 3
}

fn example5() {
    let v = vec![1, 2, 3, 4, 5];
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // Safely accesses an element of a vector using the `get` method.
    // This method returns an Option, which can be None if the index is out of bounds.
    // When you run the code, the result ==> The third element is 3
}

fn example6() {
    let mut v = vec![1, 2, 3, 4, 5];
    if let Some(last) = v.pop() {
        println!("The last element is {}", last);
    }
    println!("Vector after popping an element: {:?}", v);
    // Demonstrates removing the last element from a vector using `pop`.
    // `pop` returns an Option, which is Some if the vector was not empty.
    // When you run the code, the results ==> The last element is 5
    //                                       Vector after popping an element: [1, 2, 3, 4]
}

fn example7() {
    let mut v = vec![1, 2, 3, 4, 5];
    v[2] = v[1] + v[0];
    println!("Vector after modifying an element: {:?}", v);
    // Modifies an element of a vector by indexing.
    // When you run the code, the result ==> Vector after modifying an element: [1, 2, 3, 4, 5]
}

fn example8() {
    let v = vec!['a', 'b', 'c'];
    for i in &v {
        println!("A reference to {}", i);
    }
    // Iterates over the elements of a vector by reference.
    // When you run the code, each element will be printed on its own line.
}

fn example9() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Vector after modifying each element: {:?}", v);
    // Demonstrates iterating over the elements of a vector by mutable reference,
    // allowing modification of each element, where you add '50' to each element.
    // When you run the code, the result ==> Vector after modifying each element: [150, 82, 107]
}

fn example10() {
    let row = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("A 2D vector: {:?}", row);
    // Demonstrates creating a two-dimensional vector (a vector of vectors).
    // This is useful for creating dynamic, growable matrices.
    // When you run the code, the result ==> A 2D vector: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
}
