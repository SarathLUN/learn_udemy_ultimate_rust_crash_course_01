# Key Takeaways on Threading in Rust:

1. Rust threading is portal across various platforms, including Mac, Linux, and Windows.
2. Basic Thread Creation:
    - Import the thread module into scope.
    - Use `thread::spawn()` to create a new thread.
    - `spawn()` takes a closure with no arguments, which serves as the main function of the thread.
3. Join handles:
    - `spawn()` return a join handler.
    - use `join()` on the handler to wait for thread to complete.
    - the result of the `join` can contain either a success value return from the thread or an error if the thread panicked.
4. Thread safety consideration:
    - thread in Rust is a bit heavyweight, involving memory allocation for the thread's stack.
    - contexts switching between threads is an expensive operation.
    - consider the overheads of contexts switching when working with multiple threads.
5. Concurrency benefits:
    - threads are useful for concurrent CPU and memory task.
    - threads can run simultaneously on multiple cores, allowing for efficient parallelism and increase productivity.
6. Async/Await for I/O operations:
    - if the goal is to perform concurrent tasks like disk I/O or network I/O efficiently, consider using async/await.
    - async/await is more efficient approach for concurrently waiting for I/O operations.
7. `move` with closure:
    - in Rust, when you use the `move` keyword with a closure, it indicates that the closure takes ownership of the variables it uses from the surrounding scope. However, this does not mean that the original variable is invalidated or inaccessible after the closure created. It simply means that the closure owns a copy of the variable's value.
    - for primitive type like integers, Rust has implemented the `Copy` this means that when you move the variable into the closure, Rust create a copy of its value rather than transferring ownership outright.
    - If you want to observe the behavior where the variable is no longer accessible in the main function after being moved into the closure, you can use types that don't implement the `Copy` trait or wrap your variable in a `std::rc::Rc` (reference counting) or `std::sync::Arc` (atomic reference counting) to achieve shared ownership between threads.
