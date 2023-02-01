use std::thread;
use std::sync::mpsc::sync_channel;
use std::time::Duration;
fn main(){
    let (tx,rx) = sync_channel(1);
    thread::spawn(move ||{
        
        tx.send(String::from("Hello World1")).unwrap();
        println!("sender: Hello World1");

        tx.send(String::from("Hello World2")).unwrap();
        println!("sender: Hello World2");

        tx.send(String::from("Hello World3")).unwrap();
        println!("sender: Hello World3");
    });

    println!("sleep 2 sec");
    thread::sleep(Duration::from_secs(2));
    let content = rx.recv().unwrap();
    println!("receive content: {}",content);

    println!("sleep 2 sec");
    thread::sleep(Duration::from_secs(2));
    let content = rx.recv().unwrap();

    println!("receive content: {}",content);
    
    println!("sleep 2 sec");
    thread::sleep(Duration::from_secs(2));
    let content = rx.recv().unwrap();
    println!("receive content: {}",content);
}