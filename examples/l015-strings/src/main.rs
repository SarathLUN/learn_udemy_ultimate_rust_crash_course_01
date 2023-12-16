fn main() {
    // borrowed string slice
    let borrow_str: &str = "Hellow, World";
    println!("Borrowed String Slice: {}", borrow_str);

    // convert borrowed string slice to owned string
    let owned_str: String = borrow_str.to_string();
    println!("owned string: {}", owned_str);

    // iterating through bytes
    for byte  in owned_str.bytes() {
        println!("Byte: {}", byte);
    }

    // iterating through Unicode scalars
    for scala  in owned_str.chars() {
        println!("Unicode scala: {}",scala);
    }

    // using nth() with iterator
    let third_scalar = owned_str.chars().nth(2);
    match third_scalar {
        Some(scala) => println!("third unicode scalar: {}", scala),
        None => println!("third unicode scalar not found"),
    }
}
