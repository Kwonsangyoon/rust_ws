use std::rc::Rc;


struct Person
{
    age : i32,
}

fn main() {

    let person = Rc::new(Person{ age : 10});

    let p1 = person.clone();
    println!("{}",p1.age);
    println!("{}",Rc::strong_count(&person));
    let p2 = person.clone();
    println!("{}",p2.age);
    println!("{}",Rc::strong_count(&person));
    {
        let p3 = person.clone();
        println!("{}",p3.age);
        println!("{}",Rc::strong_count(&person));
    }
    println!("{}",Rc::strong_count(&person));




   
}
