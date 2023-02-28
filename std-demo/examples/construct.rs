struct D;
impl D{
    fn new() -> Self{
        println!("construct D");
        D{}
    }
}
fn main(){
    D::new();
}