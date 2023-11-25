fn main() {
    let mut bunnies = 2;
    let (bunnies_2, carrots) = (8, 50);
    println!("bunnies value is: {}",bunnies);
    println!("bunnies_2 value is: {}",bunnies_2);
    println!("carrots value is: {}",carrots);
    bunnies = 9; // Error!
    println!("new value of bunnies is {}", bunnies);
    
    const WARP_FACTOR: f64 = 9.9;
    println!("const WARP_FACTOR is {}", WARP_FACTOR);

}
