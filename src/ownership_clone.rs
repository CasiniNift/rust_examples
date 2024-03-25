// Introduction to the Clone Trait in Rust

/*
The Clone trait in Rust is used to explicitly duplicate values, particularly for types
that manage resources like heap memory. Implementing Clone involves more than just a
bitwise copy; it can include allocating new resources and copying data to them. This trait
allows for a flexible, explicit approach to duplicating complex types, ensuring memory safety
and preventing unintended resource duplication.

The following examples demonstrate the use of the Clone trait across various Rust types,
highlighting its necessity and functionality in safe, explicit data duplication.
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
    let s = String::from("hello");
    let s_clone = s.clone();
    println!("Original: {}, Clone: {}", s, s_clone);
    // Demonstrates cloning a String. Both s and s_clone can be used independently.
    // When you run the code, the result ==> Original: hello, Clone: hello.
}

fn example2() {
    let v = vec![1, 2, 3, 4, 5];
    let v_clone = v.clone();
    println!("Original: {:?}, Clone: {:?}", v, v_clone);
    // Showcases cloning a vector. The clone will have identical elements, but separate allocation.
    // When you run the code, the result ==> Original: [1, 2, 3, 4, 5], Clone: [1, 2, 3, 4, 5]
}

fn example3() {
    let arr = [1; 10]; // An array of ten ones.
    let arr_clone = arr.clone(); // Arrays implement Copy for types that are Copy, but Clone works regardless.
    println!("Original: {:?}, Clone: {:?}", arr, arr_clone);
    // Cloning an array. Demonstrates that Clone can also be used with types that are Copy.
    // When you run the code, the result ==> [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], Clone: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
}

fn example4() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let person_clone = person.clone();
    println!("Original: {:?}, Clone: {:?}", person, person_clone);
    // Illustrates cloning a custom struct. Requires that Person implements Clone.
    // When you run the code, the result ==> Original: Person { name: "Alice", age: 30 }, Clone: Person { name: "Alice", age: 30 }
}

#[derive(Clone, Debug)]
struct Person {
    name: String,
    age: i32,
}

fn example5() {
    let tuple = (String::from("Hello"), 42);
    let tuple_clone = tuple.clone();
    println!("Original: {:?}, Clone: {:?}", tuple, tuple_clone);
    // Demonstrates cloning a tuple containing a mix of Clone and Copy types.
    // When you run the code, the result ==> Original: ("Hello", 42), Clone: ("Hello", 42)
}

// Define examples 6 to 10 showcasing Clone with various standard library types,
// custom logic in clone implementations, and performance considerations.

fn example6() {
    let hashmap = std::collections::HashMap::from([(1, "One"), (2, "Two"), (3, "Three")]);
    let hashmap_clone = hashmap.clone();
    println!("Original: {:?}, Clone: {:?}", hashmap, hashmap_clone);
    // Cloning a HashMap. Each key-value pair is duplicated into a new allocation.
    // When you run the code, the result ==> Original: {3: "Three", 1: "One", 2: "Two"}, Clone: {3: "Three", 1: "One", 2: "Two"}
}

fn example7() {
    // Demonstrating a case with manual implementation of Clone to handle custom logic.
    let custom = CustomCloneable::new(5);
    let custom_clone = custom.clone();
    println!("Original: {}, Clone: {}", custom.value, custom_clone.value);
    // When you run the code, the result ==> Original: 5, Clone: 6
}

#[derive(Debug)]
struct CustomCloneable {
    value: i32,
}

impl CustomCloneable {
    fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Clone for CustomCloneable {
    fn clone(&self) -> Self {
        // Custom logic for cloning; could involve more than just copying the value.
        Self {
            value: self.value + 1,
        } // Example logic: increment value on clone.
    }
}

// Additional examples can explore more complex scenarios, performance implications of cloning,
// and contrasting Clone with Copy to deepen understanding.

fn example8() {
    let boxed = Box::new(10);
    let boxed_clone = boxed.clone();
    println!("Original: {}, Clone: {}", boxed, boxed_clone);
    // Demonstrates cloning a Box, which involves allocating new heap memory and copying the value.
    // When you run the code, the result ==> Original: 10, Clone: 10
}

fn example9() {
    let rc_example = std::rc::Rc::new(String::from("Rust"));
    let rc_clone = rc_example.clone();
    println!(
        "Original Rc count: {}, Clone Rc count: {}",
        std::rc::Rc::strong_count(&rc_example),
        std::rc::Rc::strong_count(&rc_clone)
    );
    // Demonstrates cloning an Rc<T>. Cloning an Rc<T> increases the reference count
    // rather than duplicating the underlying data. This is useful for shared ownership of heap data.
    // When you run the code, the result ==> Original Rc count: 2, Clone Rc count: 2
}

fn example10() {
    let complex_struct = ComplexStruct {
        data: vec![1, 2, 3],
        name: String::from("ComplexStruct"),
    };
    let complex_clone = complex_struct.clone();
    println!("Original: {:?}, Clone: {:?}", complex_struct, complex_clone);
    // Demonstrates cloning a more complex structure that includes both heap-allocated
    // data (Vec and String). This shows how Clone can be implemented and used for types
    // that combine multiple heap-allocated fields.
    // When you run the code, the result ==> Original: ComplexStruct { data: [1, 2, 3], name: "ComplexStruct" }, Clone: ComplexStruct { data: [1, 2, 3], name: "ComplexStruct" }
}

#[derive(Clone, Debug)]
struct ComplexStruct {
    data: Vec<i32>,
    name: String,
}
