extern crate queue;

use std::thread;
use std::time::Duration;
use queue::threadq::ThreadQueue;

// integration tests


/// Create a q and push / pop from if
#[test]
pub fn check_single_thread_iteration() {

    // create a queue and then try iteration from it
    let mut q: ThreadQueue<u8> = ThreadQueue::new(100 as usize);
    let p = q.pusher();

    println!("Pushing elements onto Vec");
    for i in 1..5 {
        p.push(i as u8);
    }
    println!("Q length: {}", q.len() );

    let joinh1 = thread::Builder::new().name("pusher".to_string()).spawn(move || {
        
        println!("Thread starting");

        for i in 0..5 {
            thread::sleep(Duration::from_millis(1500));
            p.push(10 + i);
        }
    });



    let it = q.iterator();
    let mut count = 0u8;
    for i in it {
        count += 1;
        println!("Next in it: {}", i );
    }

    assert_eq!(4, count);
    
}