use std::cell::Cell;

fn main(){
    let x = Cell::new(0);
    let var1 = &x;
    let var2 = &x;
    println!("x = {:?}",x);
    println!("var1 = {:?}",var1);
    println!("var2 = {:?}",var2);
    var1.set(1);
    println!("set x = 1");
    println!("x = {:?}",x);
    println!("var1 = {:?}",var1);
    println!("var2 = {:?}",var2);
    var2.set(2);
    println!("set x = 1");
    println!("x = {:?}",x);
    println!("var1 = {:?}",var1);
    println!("var2 = {:?}",var2);
}