fn main() {
    // variable lifetime within the block its declaration.
    let x = 5;
    {
        let y = 99;
        println!("{}, {}", x, y);
    } // `y` already drop-off here, so below code that try to access `y` again will be error.
      // println!("{}, {}", x, y); // Error!

    // variable shadow
    {
        let x = 99;
        println!("{}", x); // prints "99" as outer `x` is not accessible after it shadowed.
    }
    println!("{}", x); // prints "5".

    // we can change mutability of variables by shadowing them in same scope
    let mut a = 5; // a is mutable
    let a = a; // a is now immutable
    println!("{}", a);

    // we can also shadow variable to difference type in same scope
    let meme = "More cowbell!";
    // TODO: get solution from Udemy about function make_image()
    // let meme = make_image(meme); // in this example meme start with as string and then turn into image.
}

