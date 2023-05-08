#![allow(dead_code)]

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

pub fn concurrent() {
    let rcell = RefCell::new("Hello".to_string());
    let arc = Arc::new(Mutex::new(rcell));
    let internal = arc.clone();
    
    let handle = std::thread::spawn(move || {
        let cell = internal.lock().unwrap(); // Get mutex lock
        *cell.borrow_mut() = "world".to_owned(); // Mutate the RefCell
    });
    
    handle.join().unwrap(); // Wait for thread or else we will race it
    
    println!("val: {:?}", arc.lock().unwrap());
}