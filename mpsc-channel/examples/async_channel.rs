use std::thread;
use std::sync::mpsc::channel;
fn main(){
    let (tx,rx) = channel();
    thread::spawn(move ||{
        tx.send(String::from("Hello World")).unwrap();
    });
    let content = rx.recv().unwrap();
    println!("receive content: {}",content);
    
}