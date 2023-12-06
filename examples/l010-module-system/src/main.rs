use rand::Rng;
use l010_module_system::greet;

fn main() {
    greet();

    let x = rand::thread_rng().gen_range(1..=100);
    println!("x = {}",x);
}
