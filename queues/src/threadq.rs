use std::vec::Vec;
use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;

/// A queue than can be used between threads.  Can have multiple consumers ans producers.
pub struct ThreadQueue<T> {

    /// The max size the queue is allowed to grow too
    max_size: usize,

    // The vec is guarded
    arc : Arc<(Mutex<Vec<T>>, Condvar)>

}


/// Use to push items onto the queue
pub struct ThreadQueuePush<T> {
    arc : Arc<(Mutex<Vec<T>>, Condvar)>
}

/// Use to push items onto the queue
pub struct ThreadQueueIterator<T> {
    arc : Arc<(Mutex<Vec<T>>, Condvar)>,
    count: i32
}



///  Implementation of thread queue semantics
impl <T> ThreadQueue<T> {


    // Create a new instance
    pub fn new(max_size: usize) -> ThreadQueue<T> {
        // create the vec .. this is the bit that need thread "protection"
        let q:ThreadQueue<T> = ThreadQueue {
            max_size : max_size,
            arc: Arc::new( (Mutex::new(Vec::with_capacity(max_size)), Condvar::new()) )
        };
        q
    }

    /// Returns the max size of the queue
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    /// The length of the queue
    pub fn len(&self) -> usize {
        // zero-th element of pair is the lock
        let vec = self.arc.0.lock().unwrap();
        vec.len()
    }

    /// Creates a new pusher 
    pub fn pusher(&mut self) -> ThreadQueuePush<T> {
        ThreadQueuePush::new( self.arc.clone() )
    }


    // Iterator function ("is that Rust ??")
    pub fn iterator(&mut self) -> ThreadQueueIterator<T> {
        ThreadQueueIterator {
            arc:  self.arc.clone(),
            count: 13, // TODO: get size / preferred size
        }
    }
}


// We need a our own struct here .. we need the Arc<Mutex<Vec>> so we can pull data from the queue
// Also, we can't block access to the queue! (COndVar / Wait / notify ?)

/// Iterator for the queue.   Calls to "next" block until item is available
impl <T> Iterator for ThreadQueueIterator<T> {
    type Item = T;  // should be T[] ?? to get "bulk" iteration

    fn next(&mut self) -> Option<T> {

        println!("next");

        // if self.count >= 5 {
        //     println!("DEBUG / DEV - exiting");
        //     return None
        // }

        // let (lock, cond) = self.arc.lock().unwrap();
        let &(ref mutex, ref condvar) = &*self.arc;

        // let ref locks = self.arc;
        // let ref mutex = locks.0;
        // let ref condvar = locks.1;

        // Get the vec.., if has elements we can return else we wait
        let mut the_vec = mutex.lock().unwrap();

        // we have a lock result 
        let mut empty = the_vec.len() == 0;

        // this where the condavr comes in - 
        // unwrap consumes it
        while empty {
            println!("\t... waiting to start de-queuing");
            let x = condvar.wait_timeout(the_vec, Duration::from_secs(1)).unwrap();
            the_vec = x.0;
            if x.1.timed_out() {
                println!("\t... finished wait - no result");
            } else {
                // we return something ?
                empty = the_vec.len() == 0; 
            }
        }

        // should be unsafe to unwrap as we check length above
        let t = the_vec.pop().unwrap();
        println!("Done the pop");
        self.count += 1;
        Some(t)

    }

}


impl <T> ThreadQueuePush<T> {

    // Takes ownership of the mutex 
    pub(crate) fn new(v: Arc< (Mutex<Vec<T>>, Condvar)>) -> ThreadQueuePush<T> {
        ThreadQueuePush { arc : v }
    }

    // Pushes a single object onto the queue 
    // Takes ownership of T
    pub fn push(&self, t: T) {
        let mut vec = self.arc.0.lock().unwrap();
        vec.push(t);
        self.arc.1.notify_one();
    }

    // Pushes all the items from a Vec onto the queue ..
    // blocking until queue has taken all -- items are removed from source queue
    // TODO: optional timeout if no capacity?
    pub fn push_vec(&self, provided: &mut Vec<T>) {
        
        // lock it -- need capacity .. 
        // could just do a "drain " ??
        let mut vec = self.arc.0.lock().unwrap();
        while /*v.len() < self.maxlen .. 0 && */  provided.len() > 0 {
            vec.push( provided.pop().unwrap());
        }
        self.arc.1.notify_one();
    }

    // Is this possible -- ownerhsip of a slice?
    /// Pushes the slice 
    pub fn push_slice(&self, _provided: &[T]) {
    }

}




#[cfg(test)]
mod test {

    // use queue::threadq::ThreadQueue;
    use threadq::ThreadQueue;
    // use std::sync::{Arc};
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    // some test cases for my queue

    #[test]
    pub fn create_queue() {
        let _q: ThreadQueue<u8> = ThreadQueue::new(100 as usize);
        //panic!();
    }


    #[test]
    pub fn two_threads() {
        let mut q: ThreadQueue<u8> = ThreadQueue::new(100 as usize);

        // keep join handles so can "join" them
        let mut threads: Vec<JoinHandle<()>> = Vec::new();
        for i in 0..2 {

            //let xx = arc.clone();
            // let xx = q.pusher();

            let pusher = q.pusher();

            let thread_name = format!("Thread {}", i);
            
            // let joinh1 = thread::spawn(move || {
            let joinh1 = thread::Builder::new().name(thread_name).spawn(move || {
                
                println!("Thread starting");
                let my_thread = thread::current();
                let my_name = my_thread.name();


                // Hmm, so solution is this?
                // an Arc wraps  Mutex wraps an struct
                // Arc::new(Mutex::new(target)) 
                //  -- so we have shared(async) refs to the Mutex protecting to object
                // To solve ..
                //  -- a threadlocal???
                // or ignore and implement using raw mutex (or a macro ?)
                // encapsulation here would be nice ..
                // "pusher" - needs cloned arc to the mutex
                for count in 0..5 {

                    thread::sleep(Duration::from_millis(2000));
                    // do somthing with "my arc"
                    // let got_q = xx.lock().unwrap();

                    pusher.push(count);

                    // println!("{} [{:?}] Some thread ending {}", count, my_name, got_q.len());
                    println!("{} [{:?}] done pushing ",count, my_name);

                    // mutex guard dropped here (RAII)
                }

            });

            threads.push(joinh1.unwrap());  // Danger will robinson, danger .. 
         }

        for j in threads {
            j.join().unwrap();
        }

        // check that there are ten items on the queue
        assert_eq!(10, q.len());


    }

}

