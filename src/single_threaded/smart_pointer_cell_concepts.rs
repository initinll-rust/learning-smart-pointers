use std::cell::Cell;

pub fn mutate_var_without_mut_keyword() {
    println!("\nInterior Mutability Pattern - Cell<T>");
    let x = Cell::new(0);
    let x_value = x.get();
    println!("var x value before mutate: {}", x_value);
    x.set(x_value + 1);
    println!("var x value after mutate: {}", x.get());
}

pub fn mutate_struct_var_without_mut_keyword() {
    let test = Test::new();
    test.mutate_struct_var_without_mut_keyword();
}

#[derive(Debug)]
struct Test {
    x: Cell<i32>
}

impl Test {
    fn new() -> Self{
        Test {
            x: Cell::new(0)
        }
    }

    fn mutate_struct_var_without_mut_keyword(&self) {
        println!("struct Test value before mutate: {:?}", self);
        let x_value = self.x.get();
        self.x.set(x_value + 1);
        println!("struct Test value after mutate: {:?}", self);
    }
}