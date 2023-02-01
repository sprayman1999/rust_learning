use std::cell::RefCell;
use std::thread;
fn main(){
    thread_local! {
        static FOO: RefCell<u32> = RefCell::new(1);
    }
    
    FOO.with(|f|{
        println!("main thread: f value is {:?}",f);
        *f.borrow_mut() = 2;
        println!("main thread: f value is {:?}",f);
    });

    let thread1 = thread::spawn(move||{
        FOO.with(|f|{
            println!("thread1: f value is {:?}",f);
            *f.borrow_mut() = 3;
            println!("thread1: f value is {:?}",f);
        });
    });
 
    let thread2 = thread::spawn(move||{
        FOO.with(|f|{
            println!("thread2: f value is {:?}",f);
            *f.borrow_mut() = 3;
            println!("thread2: f value is {:?}",f);
        });
    });
    thread1.join().ok();
    thread2.join().ok();
    FOO.with(|f|{
        println!("main thread: f value is {:?}",f);
        *f.borrow_mut() = 3;
        println!("main thread: f value is {:?}",f);
    });
}