mod combinator;
mod deref;
mod error_handling;
mod option;
mod trait_practice;

use crate::combinator::{test_combinator, test_map};
use crate::error_handling::{test_result_type, test_unwrap_expect};

fn main() {
    option::test_option1();

    // error_handling
    match test_result_type() {
        Ok(_) => println!("test_result_type: success"),
        Err(_) => println!("test_result_type: error"),
    };

    match test_unwrap_expect() {
        Ok(_) => println!("test_unwrap_expect: success"),
        Err(_) => println!("test_unwrap_expect: error"),
    };

    // combinator
    test_map();
    test_combinator();

    // trait
    trait_practice::main();

    // RAII (ref: https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/063df5)
    deref::test_reference();
}
