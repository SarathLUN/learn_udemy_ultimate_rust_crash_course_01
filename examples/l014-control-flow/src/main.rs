fn main() {
    // if expression
    let condition = true;
    let message = if condition { "It's true!" } else { "It's false" };
    println!("{}", message);

    // unconditional loop with labeled break
    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("inner loop");
            break 'outer;
        }
    }

    // while loop
    let mut counter = 0;
    while counter < 5 {
        println!("counter: {}", counter);
        counter += 1;
    }

    // for loop with iterator and destructuring
    let numbers = vec![1,2,3,4,5];
    for &n  in numbers.iter() {
        println!("doubled: {}", n*2);
    }

    // for loop with range
    for i  in 0..3  {
        println!("index: {}", i);
    }
}
