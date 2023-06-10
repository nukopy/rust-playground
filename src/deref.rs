pub fn test_reference() {
    println!("\n===== raii::main() =====");

    // 不変参照
    test_immutable_reference();
    test_deref();
    dereference_of_mutable_reference();
}

fn test_immutable_reference() {
    let x = 10;
    let y = &x;

    println!("x: {}", x);
    // print_val(x); compile error
    print_val(y);
}

fn print_val(x: &i32) {
    println!("print_val: {}", x);
}

fn test_deref() {
    let mut x: i32 = 10;
    println!("x: {}", x); // 不変参照

    let y: &mut i32 = &mut x; // 可変参照

    deref_of_immutable_ref(y);
    println!("y: {}", y);
    deref_of_mutable_ref(y);
    println!("y: {}", y);
}

fn deref_of_immutable_ref(x: &i32) {
    println!("x: {}", *x); // 参照外し
}

fn deref_of_mutable_ref(x: &mut i32) {
    *x += 1; // 参照外し
}

fn dereference_of_mutable_reference() {
    let a: i32 = 10;
    let ref_a: &i32 = &a;

    println!("a          : {}", a);
    println!("ref of a   : {}", ref_a);
    println!("deref of a : {}", *ref_a);
    println!("a          : {}", a); // 参照外しによって所有権の移動は発生しない
}
