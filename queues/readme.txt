

* Need iterator to consider condvar
    - "close" all pushers closes the queue (RAII on pusher, "Drop" removes pusher, when no pushers, queue in "closing" state)
        - Open -> Closing -> Closed
    

