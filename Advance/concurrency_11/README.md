# Concurrency

Concurrency is the ability to run multiple tasks at the same time, which can improve the performance and responsiveness of a program. Rust provides several features and tools to help you write concurrent code that is safe and efficient. In this tutorial, I will show you how to use threads, channels, mutexes, and arcs to create a simple concurrent application in Rust.

## Threads

Threads are the basic unit of concurrency in Rust. They allow you to run multiple pieces of code in parallel, each with its own stack and local variables. To create a new thread, you can use the `thread::spawn` function and pass it a closure that contains the code you want to run on the new thread. For example, the following code creates two threads that print some messages:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread and store its handle
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Run some code on the main thread
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
```

The `thread::spawn` function returns a `JoinHandle`, which is a value that represents the ownership of the new thread. You can use the `join` method on the handle to wait for the thread to finish executing, or to get the return value of the closure if there is any. The `unwrap` method is used to handle any errors that might occur when joining the thread.

Note that when the main thread of a Rust program finishes, all the spawned threads are shut down, whether or not they have finished running. Therefore, it is important to use `join` to ensure that the threads complete their tasks before the program exits.

## Channels

Channels are a way of communicating between threads by sending and receiving messages. A channel consists of two parts: a transmitter and a receiver. The transmitter can send a value to the channel, and the receiver can wait for and retrieve the value from the channel. Rust provides a `mpsc` module (short for multiple producer, single consumer) that implements a channel that can have multiple transmitters but only one receiver. To create a channel, you can use the `mpsc::channel` function, which returns a tuple of a transmitter and a receiver. For example, the following code creates a channel and uses it to send a string from one thread to another:

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    // Create a channel and get its transmitter and receiver
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread and move the transmitter to it
    thread::spawn(move || {
        // Send a message to the channel
        let message = "Hello from the spawned thread!";
        tx.send(message).unwrap();
        println!("Message sent: {}", message);
    });

    // Receive the message from the channel on the main thread
    let message = rx.recv().unwrap();
    println!("Message received: {}", message);
}
```

The `send` method on the transmitter takes ownership of the value it sends, and returns a `Result` type that indicates whether the send was successful or not. The `recv` method on the receiver blocks the current thread until a value is available, and returns a `Result` type that contains the value or an error. The `unwrap` method is used to handle any errors that might occur when sending or receiving.

Note that you need to use the `move` keyword when passing the transmitter to the `thread::spawn` function, because the closure needs to take ownership of the transmitter. This also means that you can only send values that implement the `Send` trait, which indicates that they can be safely transferred across threads.

## Mutexes

Mutexes are a way of protecting shared data from concurrent access by multiple threads. A mutex is a data structure that wraps a value and provides a locking mechanism to control the access to the value. Only one thread can acquire the lock at a time, and any other thread that tries to access the value will have to wait until the lock is released. Rust provides a `Mutex` type in the `std::sync` module that implements this functionality. To create a mutex, you can use the `Mutex::new` function and pass it the value you want to protect. To access the value, you can use the `lock` method on the mutex, which returns a `MutexGuard`, a smart pointer that dereferences to the value and releases the lock when it goes out of scope. For example, the following code creates a mutex that wraps an integer and uses it to increment the value from multiple threads:

```rust
use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    // Create a mutex that wraps an integer
    let counter = Arc::new(Mutex::new(0));
    // Create a vector to store the thread handles
    let mut handles = vec![];

    // Spawn 10 threads and move the mutex to each of them
    for _ in 0..10 {
        // Clone the mutex reference
        let counter = Arc::clone(&counter);
        // Spawn a new thread and store its handle
        let handle = thread::spawn(move || {
            // Acquire the lock and get a mutable reference to the value
            let mut num = counter.lock().unwrap();
            // Increment the value
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value
    println!("Result: {}", *counter.lock().unwrap());
}
```

The `Arc` type is a thread-safe reference-counting pointer that allows multiple ownership of the same value across threads. You need to use `Arc` to wrap the mutex, because the `Mutex` type does not implement the `Copy` trait, which means you cannot simply clone it. The `Arc::clone` function creates a new pointer to the same value and increments the reference count. When the pointer goes out of scope, the reference count is decremented, and when it reaches zero, the value is dropped.

Note that you need to use the `unwrap` method to handle any errors that might occur when locking or unlocking the mutex. A common error is a deadlock, which happens when two or more threads are waiting for each other to release a lock, preventing them from making any progress.

## Arcs

Arcs are a way of sharing immutable data between threads without copying. An arc is a data structure that wraps a value and provides a reference-counting mechanism to keep track of the number of references to the value. When the last reference goes out of scope, the value is dropped. Rust provides an `Arc` type in the `std::sync` module that implements this functionality. To create an arc, you can use the `Arc::new` function and pass it the value you want to share. To access the value, you can dereference the arc as if it were a regular pointer. For example, the following code creates an arc that wraps a vector of strings and uses it to print the values from multiple threads:

```rust
use std::thread;
use std::sync::Arc;

fn main() {
    // Create an arc that wraps a vector of strings
    let names = Arc::new(vec!["Alice", "Bob", "Carol", "Dave"]);
    // Create a vector to store the thread handles
    let mut handles = vec![];

    // Spawn 4 threads and move the arc to each of them
    for i in 0..4 {
        // Clone the arc reference
        let names = Arc::clone(&names);
        // Spawn a new thread and store its handle
        let handle = thread::spawn(move || {
            // Get the name at the index
            let name = &names[i];
            // Print the name
            println!("Name: {}", name);
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}
```

The `Arc` type is similar to the `Rc` type that we saw in Chapter 15, but it is thread-safe, which means it implements the `Send` and `Sync` traits that allow it to be transferred and shared across threads. However, this also means that you cannot mutate the value inside an arc, because that would violate the memory safety guarantees of Rust. If you need to share mutable data, you can use a combination of `Arc` and `Mutex`, as we saw in the previous section.

## Conclusion

In this tutorial, you learned how to use threads, channels, mutexes, and arcs to write concurrent code in Rust. You also learned how Rust's ownership and type systems help you avoid common concurrency errors, such as data races and deadlocks. Rust's concurrency features are powerful and expressive, but they also require careful attention and discipline.
