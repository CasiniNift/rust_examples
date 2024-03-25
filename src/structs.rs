// Introduction to Structs in Rust
// To run the code, type "cargo run structs" in the terminal.

/*
Structs in Rust are custom data types that let you name and package together multiple related values
that make up a meaningful group. Unlike tuples, which are simple and untyped, structs allow you to create
more flexible, self-documenting code by naming the parts of the struct. This makes structs ideal for
creating complex data structures that represent objects and concepts in your domain.

In this section, you'll learn how to define structs, instantiate them, and how to use methods and associated
functions to implement behavior related to your structs. Structs are a cornerstone of Rust's type system,
enabling you to craft robust and efficient abstractions.

The examples provided will guide you through various aspects of working with structs in Rust,
from defining and instantiating structs, accessing their fields, to more advanced patterns
like using tuple structs and unit structs. By mastering structs, you will be able to build more
expressive and type-safe Rust programs.
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
    // Define a simple struct named `User` that represents a user profile.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // Instantiate a `User`.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {} | Email: {}", user1.username, user1.email);
    // Demonstrates how to create an instance of a struct and access its fields.
    // When you run the code, the result ==> User: someusername123 | Email: someone@example.com
}

fn example2() {
    // Define a tuple struct named `Color` that represents a color with RGB values.
    struct Color(i32, i32, i32);

    // Instantiate a `Color`.
    let black = Color(0, 0, 0);

    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    // Demonstrates how to use a tuple struct.
    // When you run the code, the result ==> Black: (0, 0, 0)
}

fn example3() {
    struct Color {
        red: i32,
        green: i32,
        blue: i32,
    }

    let background = Color {
        red: 255,
        green: 255,
        blue: 255,
    };

    println!(
        "Background color RGB: {}, {}, {}",
        background.red, background.green, background.blue
    );
    // Demonstrates a classic struct with named fields for specifying RGB colors.
    // When you run the code, the result ==> Background color RGB: 255, 255, 255
}

fn example4() {
    // Example of using an enum within a struct.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
    }

    struct Screen {
        components: Vec<Message>,
    }

    let screen = Screen {
        components: vec![
            Message::Quit,
            Message::Move { x: 10, y: 20 },
            Message::Write(String::from("Hello, world!")),
        ],
    };

    println!("Screen has {} components.", screen.components.len());
    // When you run the code, the result ==> Screen has 3 components.
}

fn example5() {
    // Implementing methods in structs
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    // When you run the code, the result ==> The area of the rectangle is 1500 square pixels.
}

fn example6() {
    // More complex method usage in structs
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // When you run the code, the result ==> Can rect1 hold rect2? true
}

fn example7() {
    // Demonstrates the use of struct update syntax to create new instances from existing ones.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!(
        "User2 Email: {}, Username: {}, Sign in count: {}, Active: {}",
        user2.email, user2.username, user2.sign_in_count, user2.active
    );
    // When you run the code, the result ==> User2 Email: another@example.com, Username: someusername123, Sign in count: 1, Active: true
}

fn example8() {
    // Demonstrates the use of a unit struct without any fields.
    struct Unit;

    let unit = Unit;

    println!("Unit struct instantiated.");
    // Unit structs can be useful for generics or when you need to implement a trait on something but don't need to store any data in it.
    // When you run the code, the result ==> Unit struct instantiated.
}

fn example9() {
    // Demonstrates struct ownership with heap data.
    struct Message {
        content: String,
    }

    impl Message {
        fn new(msg: &str) -> Message {
            Message {
                content: String::from(msg),
            }
        }

        fn print(&self) {
            println!("Message: {}", self.content);
        }
    }

    let my_message = Message::new("Hello, Rust!");
    my_message.print();
    // When you run the code, the result ==> Message: Hello, Rust!
}

fn example10() {
    // Demonstrates using a struct with an associated function and a method.
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let square = Rectangle::square(3);
    println!("Square: {:?}, Area: {}", square, square.area());
    // When you run the code, the result ==> Square: Rectangle { width: 3, height: 3 }, Area: 9
}
