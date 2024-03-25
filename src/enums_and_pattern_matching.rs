/*
Introduction to Enums and Pattern Matching in Rust:

Enums in Rust are a way to define a type by enumerating its possible variants.
This feature is especially powerful when combined with Rust's pattern matching
capabilities, allowing for expressive control flow that can take different actions
based on the specific variant of an enum that is encountered.

Enums are a core part of Rust's type system, enabling you to encapsulate different
types and associated data in a single, cohesive type. Pattern matching,
facilitated by the match statement or if let expressions, provides a robust
and type-safe way to work with enums, handling each possible variant explicitly.

This section will explore the definition and usage of enums, alongside examples of
pattern matching to handle various enum variants. These concepts are foundational
for writing expressive and safe Rust code, particularly when dealing with options,
errors, or any data that can have multiple forms.
*/

// Demonstrates using enums and pattern matching in Rust.
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn example1() {
    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
    // Demonstrates pattern matching with enums. This match statement handles each variant of the Direction enum.
    // When you run the code, the result ==> Going up!
}

fn example2() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::Write(String::from("hello"));
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to: {}, {}, {}", r, g, b),
    }
}
// Shows pattern matching with an enum that includes different types of associated data.
// When you run the code, the result ==> Text message: hello

fn example3() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    match some_number {
        Some(x) => println!("The number is: {}", x),
        None => println!("No number"),
    }

    match absent_number {
        Some(x) => println!("The number is: {}", x),
        None => println!("No number"),
    }
    // Demonstrates using the Option enum to handle values that might be absent.
    // When you run the code, the result ==> The number is: 5 followed by No number
}

fn example4() {
    let ok_result: Result<i32, &str> = Ok(10);
    let error_result: Result<i32, &str> = Err("An error occurred");

    match ok_result {
        Ok(value) => println!("Success with value: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match error_result {
        Ok(value) => println!("Success with value: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    // Shows handling of Result enum, which can encapsulate a successful outcome with a value or an error.
    // When you run the code, it will print a success message for ok_result and an error message for error_result.
}

fn example5() {
    enum Action {
        Drive,
        Turn(Direction),
        Stop,
    }

    let actions = vec![Action::Drive, Action::Turn(Direction::Left), Action::Stop];

    for action in actions {
        match action {
            Action::Drive => println!("Start driving"),
            Action::Turn(dir) => match dir {
                Direction::Left => println!("Turn left"),
                _ => println!("Turn"),
            },
            Action::Stop => println!("Stop driving"),
        }
    }
    // Demonstrates nesting enums within enums and using nested pattern matching.
    // When you run the code, it will sequence through the actions: driving, turning left, then stopping.
}

fn example6() {
    let some_value = Some(3);

    if let Some(x) = some_value {
        println!("Matched, x = {}", x);
    } else {
        println!("Did not match");
    }
    // Introduces 'if let' for a concise way to handle enums when only one variant is of interest.
    // When you run the code, the result ==> Matched, x = 3
}

fn example7() {
    let numbers = vec![Some(2), None, Some(3)];

    for number in numbers {
        match number {
            Some(i) if i % 2 == 0 => println!("Even number: {}", i),
            _ => (),
        }
    }
    // Demonstrates using match guards to add conditions to branches in pattern matching.
    // When you run the code, it identifies and prints even numbers: Even number: 2
}

fn example8() {
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // Using 'if let' with an enum variant that has associated data.
    // When you run the code, it will print the state of the quarter.
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn example9() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // Demonstrates the use of pattern matching with literals.
    // When you run the code, there's no match, so no output is generated due to the wildcard pattern.
}

fn example10() {
    let robot_name: Option<&str> = Some("Bender");

    match robot_name {
        Some(name) => println!("Found a robot named {}", name),
        _ => println!("No robot found"),
    }

    if let Some(name) = robot_name {
        println!("Found a robot named {}", name);
    } else {
        println!("No robot found");
    }
    // Showcases using both match and if let to work with an Option enum.
    // When you run the code, it will print the robot's name twice, demonstrating two ways to achieve the same result.
}
