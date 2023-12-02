fn main() {
    let enigma: i16;
    // if true{
    //     enigma = 42; // compiler can't garante that `enigma` is initialized
    // }

    // println!("{}", enigma); // this code won't event compiled, because `enigma` used here but it isn't initialized

    if true{
        enigma = 42; // compiler can't garante that `enigma` is initialized
    } else {
        enigma = 7;
    }
    println!("{}", enigma); // this works
}
