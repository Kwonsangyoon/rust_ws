//use std::io;

fn main() {
    let s1 =String::from("hello");
    let len = take_ownership(&s1);
    println!("{},world is len {}",s1,len);
    
}

fn take_ownership(str : &String) -> usize
{
    println!("{str}");
    return str.len();
}