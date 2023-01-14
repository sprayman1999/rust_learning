use std::{fs::File, env, io::{Write, Read},fs::{self, OpenOptions}};


fn main(){
    let cmd_args: Vec<_> = env::args().collect();
    let filepath = cmd_args.get(1).unwrap();
    {
        println!("write \'HelloWorld\' to {}",filepath);
        let mut file_handle = OpenOptions::new().read(true).create(true).write(true).open(filepath).unwrap();
        file_handle.write(b"HelloWorld").unwrap();
    }
    {
        let mut file_content: String = String::new();
        println!("open file: {}",filepath);
        let mut file_handle = fs::File::open(filepath).unwrap();
        file_handle.read_to_string(&mut file_content).unwrap();
        println!("file_content: {}",file_content);
    }
    println!();
}
