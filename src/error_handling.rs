use std::fs::File;
use std::io::ErrorKind;

#[allow(dead_code)]
pub fn main() {
    // test panic
    let is_panic = true;
    raise_panic(is_panic);
}

#[allow(dead_code)]
pub fn raise_panic(is_panic: bool) {
    if is_panic {
        panic!("raise panic");
    }

    println!("not panic");
}

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

pub fn test_result_type() -> Result<(), std::io::Error> {
    let f = File::open("./samples/hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("f: {:?}", f);

    // write text
    // let text = "Hello, world!";
    // match f.write_all(text.as_bytes()) {
    //     Ok(_) => println!("successfully wrote to file"),
    //     Err(e) => println!("error writing to file: {}", e),
    // }

    Ok(())
}

pub fn test_unwrap_expect() -> Result<(), std::io::Error> {
    let x = Some(2);
    let x = x.unwrap();
    println!("x: {}", x);

    // let y = None;
    // let y: i32 = y.expect("Failed to unwrap y!!!!!");
    // println!("y: {}", y);

    // let z = None;

    Ok(())
}
