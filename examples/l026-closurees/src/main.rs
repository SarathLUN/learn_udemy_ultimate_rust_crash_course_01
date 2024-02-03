
fn main() {
    // Example of Simple Closure
    let add = |x,y| {x + y};
    let result = add(8, 2);
    println!("result: {}",result);
    
    // Closures Without Parameters
    let no_param = || {println!("closure with no param.")};
    no_param();

    // Borrowing References in Closures
    let x = String::from("test");
    let s = String::from("strawberry");
    let f = || {
        println!("closure borrow `s`: {}", s);
        println!("closure try to borrow `x`: {}", x);
        println!("closure try to borrow `result`: {}", result);
    };
    f();
    println!("`s` is still valid: {}", s);

    // Move Semantics in Closures
    let f2 = move || {
        println!("closure move `s`: {}", s);
    };
    f2();
    // println!("try to access value of `s` after moved: {}", s); // compile time error 

    // Functional programming with closures
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<_> = numbers.iter().map(|&x| x * 3).filter(|&x| x > 10).collect();
    println!("Mapped and filtered result: {:?}", result);

    let sum: i32 = numbers.iter().filter(|&&x| x > 10).fold(0, |acc, x| acc + x);
    println!("Sum of filtered values: {}", sum);

}
