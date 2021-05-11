struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn run() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("c data: {}", c.data);
    drop(c);
    //println!("c data: {}", c.data);
}