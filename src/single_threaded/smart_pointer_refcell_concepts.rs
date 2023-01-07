use std::cell::RefCell;

pub fn mutate_var_without_mut_keyword_1() {
    println!("\nInterior Mutability Pattern - RefCell<T>");
    let x = RefCell::new(0);
    let mut x_value = x.borrow_mut();
    println!("var x value before mutate: {:?}", x_value);
    *x_value = *x_value + 1;
    drop(x_value);
    println!("var x value after mutate: {:?}", *x.borrow());
}

pub fn mutate_var_without_mut_keyword_2() {
    println!("\nInterior Mutability Pattern - RefCell<T>");
    let x = RefCell::new(0);
    println!("var x value before mutate: {:?}", x);
    *x.borrow_mut() += 1;
    println!("var x value after mutate: {:?}", x);
}

pub fn mutate_struct_var_without_mut_keyword_1() {
    let test = Test::new();
    test.mutate_struct_var_without_mut_keyword_1();
}

pub fn mutate_struct_var_without_mut_keyword_2() {
    let test = Test::new();
    test.mutate_struct_var_without_mut_keyword_2();
}

#[derive(Debug)]
struct Test {
    x: RefCell<i32>
}

impl Test {
    fn new() -> Self{
        Test {
            x: RefCell::new(0)
        }
    }

    fn mutate_struct_var_without_mut_keyword_1(&self) {
        println!("struct Test value before mutate: {:?}", self);
        let mut x_value = self.x.borrow_mut();
        *x_value = *x_value + 1;
        drop(x_value);
        println!("struct Test value after mutate: {:?}", self);
    }

    fn mutate_struct_var_without_mut_keyword_2(&self) {
        println!("struct Test value before mutate: {:?}", self);
        *self.x.borrow_mut() += 1;
        println!("struct Test value after mutate: {:?}", self);
    }
}