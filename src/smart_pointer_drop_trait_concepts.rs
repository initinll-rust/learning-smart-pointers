struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop Trait triggered - Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn drop_resource_automatically() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff")
    };

    let _d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("\nAuto Drop");
    println!("CustomSmartPointers created.");
}

pub fn drop_resource_manually() {
    let _c = CustomSmartPointer {
        data: String::from("some data")
    };

    println!("\nManual Drop");
    println!("CustomSmartPointers created.");
    drop(_c);
    println!("CustomSmartPointer dropped before the end of main.");
}