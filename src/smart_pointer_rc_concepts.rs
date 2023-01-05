use std::rc::Rc;

pub fn rc_list_immutable() {
    // rc is immutable
    use crate::smart_pointer_rc_concepts::List::{Cons, Nil};
    
    println!("\nReference counting and clonning - list");

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("Reference count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(10, Rc::clone(&a));
    println!("Reference count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Cons(20, Rc::clone(&a));
        println!("Reference count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Reference count after droping c = {}", Rc::strong_count(&a));
}

pub fn rc_string_immutable() {
    // rc is immutable

    println!("\nReference counting and clonning - string");

    let data = "Rc examples".to_string();

    let a: Rc<String>  = Rc::new(data);
    println!("Reference count after creating a = {}", Rc::strong_count(&a));

    let _b: Rc<String> = Rc::clone(&a);
    println!("Reference count after creating b = {}", Rc::strong_count(&a));
    
    let _c: Rc<String> = Rc::clone(&a);
    println!("Reference count after creating c = {}", Rc::strong_count(&a));
}


#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}