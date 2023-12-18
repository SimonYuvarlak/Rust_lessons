use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Threads -----
    // Threads allow programs to use different processor cores of the computer simultaneously.
    // This enables your code to perform multiple tasks in parallel instead of sequentially.
    // Thread usage can lead to issues like race conditions and deadlocks.

    // Threads are created using thread::spawn() and it takes a closure.
    // The code inside that closure runs on a new thread.
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("thread number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("main number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // The result you get each time may vary depending on how your operating system allocates cores.

    // In the above example, our thread's work was cut off because the main finished first.
    // When the main finishes, all running threads are also stopped.
    // As a solution, you can follow the approach below.
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         // println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     // println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap(); // This line terminates our thread.
    // If we place the above line before the for loop in main, the thread finishes all its work first, then main runs.
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // Move Closure and Thread
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();
    // The above code will throw an error because `v` is referenced in the closure, but the thread, while working,
    // does not want to operate on a reference whose validity it can't be sure of.
    // In this case, we can use the `move` keyword to give the ownership of `v` to the thread.
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();

    // Threads and Messages -----
    // Opening a channel
    // let (tx, rx) = mpsc::channel();
    // Tx is the sending part, rx is the receiving part.
    // We captured the values returned from the channel() function as a tuple.

    // -----

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    // Above, we opened a channel and in our thread, we sent the `val` value through our channel.
    // We used the send method for this.
    // The send method returns a Result.

    // We can receive the values we sent as follows.
    // The receiver below is the main thread.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // Dropped it in the river
    // });

    // let received = rx.recv().unwrap(); // Picked it up from the river
    // println!("Got: {}", received);

    // The following code will throw an error.
    // Because we're trying to print the value `val` inside our thread after sending it through the channel.
    // Rust doesn't allow this because the value `val` might be dropped or we might encounter an issue in the channel.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("val is {}", val);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // In the following example, the thread sends values one by one.
    // In the main thread, we used rx like an iterator.
    // Main printed the values as they arrived.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // // If we wanted to send many messages simultaneously from multiple threads:
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
