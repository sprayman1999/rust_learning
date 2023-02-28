use std::ops::Drop;
struct D(i32);
impl Drop for D {
    fn drop(&mut self) {
        println!("destruct {}",self.0);
    }
}
fn main(){
    let _x = D(1);
    {
        let _y = D(2);
    }
}