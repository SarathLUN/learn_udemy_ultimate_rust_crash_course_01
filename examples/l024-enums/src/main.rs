use std::{i32, fs::File};

fn option_example(x: Option<i32>) {
    if let Some(value) = x {
        println!("Got a value: {}", value);
    } else {
        println!("Got none");
    }

    match x {
        Some(value) => println!("Matched: {}", value),
        None => println!("Matched None"),
    }
}

fn result_example() {
    
    let file_result = File::open("example.txt");

    // test #1: using if
    // if file_result.is_ok(){
    //     let file = file_result.unwrap();
    //     println!("File opened successful: {:?}", file);
    // } else {
    //     println!("Failed to open file");
    // }

    // test #2: using match
    // match file_result {
    //     Ok(file) => println!("Matched ok: {:?}", file),
    //     Err(err) => println!("Matched Err: {:?}", err),
    // }
    
    //test #3: using unwrap() with Result (may crash if Err)
    // let file = file_result.unwrap();
    // println!("Unwrap file: {:?}", file);

    //test #4: using expect() with Result (crash with custom context if Err)
    let file = file_result.expect("Failed to open file");
    println!("Expect file: {:?}", file);
}

fn main() {
    // example using option
    let some_value = Some(42);
    let none_value: Option<i32> = None;

    option_example(some_value);
    option_example(none_value);

    // example using Result
    result_example();
}
