// fn max_num(x: &i32, y: &i32) -> &i32{
fn max_num<'a>(x: &'a i32, y: &'a i32) -> &'a i32{
    if x > y {
        &x
    }else{
        &y
    }
}
fn min_num<'a, 'b>(x: &'a i32, y: &'a i32) -> &'b i32
    where 'a:'b
{
    if x < y {
        &x
    }else{
        &y
    }
}
fn main(){
    let x = 1;
    let y = 2;
    let max = max_num(&x,&y);
    println!("{}",max);
    let min = min_num(&x,&y);
    println!("{}",min);
}