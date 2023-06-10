mod combinator;
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
}
