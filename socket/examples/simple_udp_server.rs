use std::{ 
    net::{UdpSocket},
};
fn main(){
    let udp_socket = UdpSocket::bind("127.0.0.1:45678").unwrap();
    loop {
        let mut content_from_client: [u8;4096] = [0;4096];
        match udp_socket.recv_from(&mut content_from_client){
            Ok((length,client_addr)) => {
                println!("{} send msg to server!",client_addr);
                for pos in 0..length{
                    print!("{}",content_from_client[pos] as char);
                }
                if length > 0{
                    udp_socket.send_to("Hello Rust UDP Server!".as_bytes(),client_addr).unwrap();
                }
            },
            Err(_) => break
        };
    }
}