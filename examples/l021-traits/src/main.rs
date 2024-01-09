use core::str;
use std::u8;

trait Noisy {
    fn get_noise(&self) -> &str;
}

struct RedFox;

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "meow"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("Noise: {}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "beep"
    }
    
}

fn main() {
    let red_fox = RedFox;
    print_noise(red_fox);

    let byte_noise: u8 = 42;
    print_noise(byte_noise);
}
