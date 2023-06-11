struct MyBox;

impl Drop for MyBox {
    fn drop(&mut self) {
        println!("MyBox is dropped");
    }
}
