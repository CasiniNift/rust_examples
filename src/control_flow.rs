// Introduction to control flows.
// To run the code, type "cargo run control" in the termnial.

/*
Control flow in Rust allows you to branch your code execution paths and create loops to repeat tasks,
which is fundamental to almost all programming tasks.
We will cover if statements, match expressions, and various loops (loop, while, and for).
Here's how we can structure our examples to provide a comprehensive understanding of control flow in Rust.
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
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // This is common "if" example.
    // When you run the code, the result ==> condition was false.
}

fn example2() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    // We are setting the condition to true if the number is 5.
    // When you run the code, the result ==> The value of number is: 5
    // If the condition was set to false, the value of the number would be 6
}

fn example3() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    // Here, you are adding 1 to the counter in a loop.
    // Once the counter gets to 10 you multiply it by 2.
    // At that point, the loop finishes.
    // When you run the code, the result ==> The result is 20
}

fn example4() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    // We are doing a "while loop".
    // We are starting with "3" and subtracting 1 with each iteration.
    // We stop once we get to zero (that is when we print).
    // When you run the code, the result will print a countdown from 3 to LIFTOFF!!!
}

fn example5() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // When you run the code, it iterates through the array, printing each value.
}

fn example6() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    // Demonstrates a reverse countdown using a for loop, similar to example 4 but more concise.
    // rev means that you work your way backwards.
}

fn example7() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    // Showcases nested loops and breaking out of them with labels.
}

fn example8() {
    let number = 6;
    match number {
        1 => println!("It's one!"),
        2 => println!("It's two!"),
        3 => println!("It's three!"),
        _ => println!("It's something else!"),
    }
    // Demonstrates the match statement, similar to a switch-case in other languages.
}

fn example9() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
    // When you run the code, the result ==> not three
    // Demonstrates `if let` as a concise way to match one pattern and ignore the rest.
}

fn example10() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // Demonstrates matching multiple patterns with | for OR functionality.
    // When you run the code, the result ==> one or two
}
