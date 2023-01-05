pub fn normal_reference() {
    // normal reference & dereference example
    let x1 = 5;
    let y1 = &x1;

    println!("\nnormal_reference");
    println!("x1 => value - {} | address - {:p}", x1, &x1);
    println!("y1 => value - {} | address - {:p}", *y1, y1);
}

pub fn box_reference() {
    // using Box<T> Like a reference & dereference example

    use crate::smart_pointer_box_concepts::List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x2 = 5;
    let y2 = Box::new(x2);

    println!("\nbox_reference");
    println!("list - {:?}", list);
    println!("x2 => value - {} | address - {:p}", x2, &x2);
    println!("y2 => value - {} | address - {:p}", *y2, y2);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}