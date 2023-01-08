use std::rc::Rc;
use std::cell::RefCell;

pub fn mutate_var_without_mut_keyword_1() {
    println!("\nInterior Mutability Pattern - Rc<RefCell<T>>");
    let x: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));

    let x1 = Rc::clone(&x);
    let x2 = Rc::clone(&x);

    *x.borrow_mut() += 10;
    println!("var x: {:?}", x);
    println!("var x1: {:?}", x1);
    println!("var x2: {:?}", x2);
}