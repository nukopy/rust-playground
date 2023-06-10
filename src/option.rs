/*
enum Option<T> {
    Some(T),
    None,
}
 */

pub fn test_option1() {
    let some1: Option<i32> = Some(5);
    let some2: Option<&str> = Some("Hello");
    let some3: Option<String> = Some(String::from("Hello"));

    println!("some1: {:?}", some1);
    println!("some2: {:?}", some2);
    println!("some3: {:?}", some3);

    let mut x: Option<i8> = None;
    println!("x: {:?}", x);
    x = Some(5);
    println!("x: {:?}", x);

    // 以下はエラー
    // error[E0369] cannot add `i8` to `Option<i8>`
    // println!("x: {:?}", x + 3);

    // calc
    let x = calc(Some(5), 3, Ops::Add);
    println!("x: {:?}", x);
    let x = calc(Some(5), 3, Ops::Sub);
    println!("x: {:?}", x);
    let x = calc(None, 3, Ops::Add);
    println!("x: {:?}", x);

    // Some の match 式所有権が移動している
    match式では所有権が移動している();
}

enum Ops {
    Add,
    Sub,
}

fn calc(x: Option<i32>, y: i32, ops: Ops) -> Option<i32> {
    match x {
        None => None,
        Some(z) => match ops {
            Ops::Add => Some(z + y),
            Ops::Sub => Some(z - y),
        },
    }
}

fn match式では所有権が移動している() {
    // i32 型はコピートレイトが実装されているので所有権の移動ではなく、コピーが行われる
    let x = Some(5);
    match x {
        Some(i) => println!("i: {}", i), // 所有権の移動
        None => println!("None"),
    }
    // 以下はエラー
    // error[E0382]: borrow of moved value: `x`
    println!("x: {:?}", x);

    // String 型はコピートレイトが実装されていないので所有権の移動が行われる
    let x = Some(String::from("Hello"));
    match x {
        Some(s) => println!("s: {}", s), // 所有権の移動
        None => println!("None"),
    }
    // 以下はエラー
    // error[E0382]: borrow of moved value: `x`
    // println!("x: {:?}", x);
}
