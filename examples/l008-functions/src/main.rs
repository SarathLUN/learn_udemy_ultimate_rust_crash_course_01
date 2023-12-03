fn main() {
    let x = do_stuff(2.0, 12.5); // rust not support args name, so we must provide in correct
                                 // order.
    println!("do_stuff = {}", x); // `println!` is a macro.
}

fn do_stuff(qty: f64, oz: f64) -> f64 {
    // return qty * oz;
    qty * oz // the final line of the function block without `;` consider as return statement in
             // Rust. Means we don't need to use keywork `return`.
}
