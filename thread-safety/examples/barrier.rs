use std::{sync::{Arc, Barrier}, thread};

fn main(){
    let barrir = Arc::new(Barrier::new(10));
    let mut handlers = vec![];
    for _ in 0..10 {
        let c = barrir.clone();
        let t = thread::spawn(move ||{
            println!("before wait");
            c.wait();
            println!("after wait");
        });
        handlers.push(t);
    }
    for h in handlers{
        h.join().ok();
    }
}