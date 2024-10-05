fn main() {
    let mut x = Box::new(10);
    println!("x:{}",x);

    *x=20;
    println!("x:{}",x);


}
