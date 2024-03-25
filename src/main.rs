use std::env;

// Here are all of the modules.
mod arrays;
mod control_flow;
mod enums_and_pattern_matching;
mod error_handling;
mod functions;
mod ownership_clone;
mod ownership_move;
mod ownership_reference_borrowing;
mod ownrship_copy;
mod slices;
mod strings_and_str;
mod strings_literals;
mod structs;
mod tuples;
mod variables_and_mutability;
mod vectors;

// Add other modules here

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "arrays" => arrays::run_examples(), // Here, you can writte "cargo run arrays" to excecute the code in the arrays file.
            "variables" => variables_and_mutability::run_examples(),
            "tuples" => tuples::run_examples(),
            "vectors" => vectors::run_examples(),
            "slices" => slices::run_examples(),
            "strings" => strings_and_str::run_examples(),
            "literals" => strings_literals::run_examples(),
            "functions" => functions::run_examples(),
            "control" => control_flow::run_examples(),
            "move" => ownership_move::run_examples(),
            "copy" => ownrship_copy::run_examples(),
            "clone" => ownership_clone::run_examples(),
            "ref" => ownership_reference_borrowing::run_examples(),
            "structs" => structs::run_examples(),
            "enums" => enums_and_pattern_matching::run_examples(),
            "error" => error_handling::run_examples(),
            // Add other cases here
            _ => println!("Section not recognized. Please specify a valid section."),
        }
    } else {
        println!("Please specify a section to run. For example: `cargo run variables`");
    }
}
