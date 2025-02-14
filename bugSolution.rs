fn main() {
    let mut x = 5;

    // Solution 1: Using a mutable reference with a single owner
    { 
        let y = &mut x; 
        *y = 10; 
    }

    // Solution 2: Using interior mutability (RefCell)
    use std::cell::RefCell;
    let x = RefCell::new(5);
    {
        let mut y = x.borrow_mut();
        *y = 10;
    }
    let z = x.borrow();
    println!("x = {}", *z); // x is now 10
    // Or using a mutex if you need to deal with threads
    use std::sync::{Arc, Mutex};
    let x = Arc::new(Mutex::new(5));
    {
        let mut y = x.lock().unwrap();
        *y = 10;
    }
    let z = x.lock().unwrap();
    println!("x = {}", *z); // x is now 10
} 