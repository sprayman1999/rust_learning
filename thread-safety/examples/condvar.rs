use std::sync::{Mutex, Condvar};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
fn main(){
    let pair = Arc::new((Mutex::new(false),Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        let &(ref lock,ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("child thread {}",*started);
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("before wait {}",*started);
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("after wait {}",*started);
}