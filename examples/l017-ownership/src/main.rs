fn main() {
    println!("Hello, world!");
    // move ownership
    let s1 = String::from("Hello");
    let s2 = s1; // ownership move to s2, so s1 is invalidated.
    // uncommenting the line below would result in a compile error
    // println!("s1: {}", s1);
    println!("s2 get value from s1: {}", s2);
    // clone ownership
    let s3 = s2.clone(); // create a copy of s2 using clone()
    println!("s3: {} clone from s2", s3); 
    println!("s2: {} is still valid.", s2); // s2 is still valid

    // move ownership in a function
    let s1 = String::from("World");
    // do_stuff(s1); // ownership moved to do_stuff function.
    println!("the value from function do_stuff() is: {}", do_stuff(s1));
    // uncommenting the line below would result in a compile error
    // println!("after moved ownership, s1 is: {}",s1);


}

fn do_stuff(mut s: String) -> String {
    // function takes ownership of s
    s.push_str(", Rust!");
    s // ownership moved back to the caller.
}
