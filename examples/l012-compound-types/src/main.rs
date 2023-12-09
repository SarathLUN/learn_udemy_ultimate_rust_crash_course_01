use std::{char, i32};

fn main() {
    // tuple
    let my_tuple: (i32, f64, char) = (42, 8.14, 'a'); 

    println!("tuple member 1: {}", my_tuple.0);

    // destructuring tuple with parttern
    let (first, second, third) = my_tuple;
    println!("second member of tuple is: {}", second);
    println!("first and third member of tuple are: {}, {}", first, third);

    // array
    let my_array: [i32; 4] = [1,2,3,4];
    // accessing array element 
    println!("array element at index 2 is: {}", my_array[2]);
    // print all element of the array
    println!("My array elements:");
    for el in my_array.iter(){
        println!("{}", el);
    }

}
