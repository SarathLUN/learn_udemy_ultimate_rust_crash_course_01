fn main() {
    let mut original_value = String::from("Hello, Rust!");
    // immutable reference
    let reference_to_value = &original_value;
    println!("reference: {}", reference_to_value);
    println!("original_value: {}", original_value);
    // mutable reference
    let mutable_reference = &mut original_value;
    mutable_reference.push_str(" Welcome!");
    println!("modified value: {}", mutable_reference);
    // de-referencing a mutable reference explicitly
    (*mutable_reference).push_str(" More!");
    println!("de-referencing value: {}", original_value); // de-referencing and modifying the value
    // ownership is not moved, and we can still use the original value
    println!("original value: {}", original_value);

}
