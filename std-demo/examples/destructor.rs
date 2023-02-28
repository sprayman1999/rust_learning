use std::ops::Drop;
use std::mem::drop;
struct D(i32);
impl Drop for D {
    fn drop(&mut self) {
        println!("destruct {}",self.0);
    }
}
fn main(){
    let _x = D(1);
    let _y = D(2);
    drop(_y);
    {
        let _z = D(3);
    }

}