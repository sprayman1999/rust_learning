use std::{ 
    net::{TcpListener},
    thread, io::{Write, Read}
};
fn main(){
    let tcp_server = TcpListener::bind("127.0.0.1:45678").unwrap();
    for income in tcp_server.incoming(){
        thread::spawn(||{
            let mut client = income.unwrap();
            println!("{} connect server!",client.peer_addr().unwrap());
            loop {
                let mut content_from_client: [u8;4096] = [0;4096];
                match client.read(&mut content_from_client){
                    Ok(length) => {
                        for pos in 0..length{
                            print!("{}",content_from_client[pos] as char);
                        }
                        if length > 0{
                            client.write("Hello Rust TCP Server!".as_bytes()).unwrap();
                        }
                    },
                    Err(_) => break
                };
            }
        });
    }
}