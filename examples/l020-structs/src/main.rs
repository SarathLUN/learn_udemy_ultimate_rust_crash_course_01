use core::str;
use std::u8;

// define a RedFox struct with fields
struct RedFox {
    name: String,
    age: u8,
    color: String,
}
// implemation block for RedFox methods and associated functions
impl RedFox {
    // associated functions act as constructor
    fn new(name: &str, age: u8, color: &str) -> Self {
        Self {
            name: String::from(name),
            age,
            color: String::from(color),
        }
    }
    // methods to display information about RedFox
    fn display_info(&self) {
        println!(
            "RedFox name: {}, age: {}, color: {}",
            self.name, self.age, self.color
        );
    }
}
fn main() {
    // instantiate RedFox using associated functions
    let my_red_fox = RedFox::new("Rusty", 8, "Red");
    // associate fields and call methods using dot syntax
    println!("Age of {} is {}", my_red_fox.name, my_red_fox.age);
    my_red_fox.display_info();
}
