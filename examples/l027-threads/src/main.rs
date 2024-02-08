use std::thread;

fn main() {
    println!("start the main routine.");
    // create variable that implement the `Copy` trait.
    let mut var_main = 1;
    // create variable that not implement the `Copy` trait.
    let mut main_var_2 = String::from("original value");

    // basic threads creation
    let my_thread = thread::spawn(move || {
        // Thread's main function logic here
        println!("start my_thread.");
        // accessing var_main in the thread.
        println!("accessing var_main in side the thread: {}",var_main);
        // try to change value of var_main in thread.
        var_main = 2;
        println!("changed value of var_main and accessing within the thread: {}",var_main);
    });
    println!("main function before join thread.");
    println!("accessing var_main after thread before join: {}",var_main);
    //TODO: try to change value of var_main after thread before join
    var_main = 3;
    // Wait for the thread to finish
    my_thread.join().unwrap();
    println!("main function after join thread.");
    println!("accessing var_main after thread after join: {}",var_main);
    //TODO: try to change value of var_main after thread after join

    // create another thread that move the main_var_2
    let my_move_thread = thread::spawn(move || {
        println!("start my_move_thread");
        println!("accessing my_move_thread inside the thread: {}", main_var_2);
        // changing value of my_move_thread inside the thread
        main_var_2 = String::from("changed value");
        println!("accessing my_move_thread inside thread after changed value: {}", main_var_2);
    });
    println!("main function before joining back the my_move_thread");

    // Try to access main_var_2 after the thread creation but before joining.
    // uncomment below code: This should fail if main_var_2 is moved into the thread closure.

    // println!("accessing main_var_2 after thread creation but before join: {}", main_var_2);

    my_move_thread.join().unwrap();

    println!("main function after joining my_move_thread.");

    // uncomment below code: to test accessing main_var_2 after the thread has completed.
    // println!("accessing main_var_2 after thread after join: {}", main_var_2);
}
