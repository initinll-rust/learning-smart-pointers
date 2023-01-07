use std::{ops::Deref, fmt::Pointer};

pub fn custom_box_reference() {
    // using custom MyBox<T> Like a reference & implementing Deref and Pointer trait
    let x3 = 5;
    let y3 = MyBox::new(x3);

    println!("\ncustom_box_reference");
    println!("x3 => value - {} | address - {:p}", x3, &x3);
    println!("y3 => value - {} | address - {:p}", *y3, y3);
}

pub fn deref_coercion() {
    let name = MyBox::new(String::from("Rust"));
    hello(&name);
}

fn hello(name: &str) {
    println!("\nderef_coercion");
    println!("Hello, {name}!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// required so that we can deref like Box smart pointer using * deref operator
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// required so that we can display pointer address in prinln {:p}
impl<T> Pointer for MyBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ptr = self as *const Self;
        Pointer::fmt(&ptr, f)
    }
}