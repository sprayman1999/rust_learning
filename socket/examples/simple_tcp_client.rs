use std::net::TcpStream;
use std::io::Write;
use std::sync::mpsc::{Receiver, self};
use std::time::Duration;
use std::io::Read;
use std::thread;
fn spawn_stdin_channel() -> Receiver<String>{
    let (sender,receiver) = mpsc::channel::<String>();
    thread::spawn(move ||{
        loop{
            let mut line: String = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            sender.send(line).unwrap();
        }

    });
    receiver
}
fn main(){
    let mut tcp_client = TcpStream::connect("127.0.0.1:45678").expect("Error: Connect server failed.");
    let stdin_channel = spawn_stdin_channel();
    tcp_client.set_read_timeout(Some(Duration::new(1, 0))).expect("Error: Set read timeout failed!");
    loop {
        let mut receiver_content: [u8; 4096] = [0; 4096];
        match stdin_channel.try_recv() {
            Ok(content_from_stdin) => {
                tcp_client.write(content_from_stdin.as_bytes()).expect("Error: Send Msg failed!");
            },
            Err(_) => {}
        }
        
        match tcp_client.read(&mut receiver_content){
            Ok(read_count) => {
                for pos in 0..read_count{
                    print!("{}",receiver_content[pos] as char);
                }
            },
            Err(_) => {}
        }
    }
}