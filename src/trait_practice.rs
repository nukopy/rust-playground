use serde::{Deserialize, Serialize};

trait Geometry<T>: std::fmt::Debug {
    fn area(&self) -> T;

    // name のデフォルト実装
    fn name(&self) -> &str {
        "Geometry"
    }
}

/* こっちだとコンパイルエラー
trait Geometry<T> {
    fn area(&self) -> T;

    // name のデフォルト実装
    fn name(&self) -> &str {
        "Geometry"
    }
}
*/

// Rectangle
#[derive(Serialize, Deserialize, Debug)]
struct Rectangle {
    x: f64,
    y: f64,
}

impl Rectangle {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Geometry<f64> for Rectangle {
    fn area(&self) -> f64 {
        self.x * self.y
    }

    fn name(&self) -> &str {
        "Point"
    }
}

// Triangle
#[derive(Serialize, Deserialize, Debug)]
struct Triangle {
    height: f32,
    base: f32,
}

impl Triangle {
    fn new(height: f32, base: f32) -> Self {
        Self { height, base }
    }
}

impl Geometry<f32> for Triangle {
    fn area(&self) -> f32 {
        self.height * self.base * 0.5
    }

    fn name(&self) -> &str {
        "Triangle"
    }
}

impl From<f64> for Rectangle {
    fn from(input: f64) -> Self {
        Self { x: input, y: input }
    }
}

impl From<f32> for Triangle {
    fn from(input: f32) -> Self {
        Self {
            height: input,
            base: input,
        }
    }
}

// ジェネリック型のトレイト境界を定義する 3 つの方法
#[warn(dead_code)]
fn draw<T>(geometry: &impl Geometry<T>) {
    println!("draw: {:?}", geometry);
}

#[warn(dead_code)]
fn draw1<T: Geometry<f64>>(geometry: &T) {
    println!("draw1: {:?}", geometry);
}

#[warn(dead_code)]
fn draw2<T>(geometry: &T)
where
    T: Geometry<f64>,
{
    println!("draw2: {:?}", geometry);
}

pub fn main() {
    println!("\n===== trait_practice::main() =====");

    // Rectangle
    let r = Rectangle::new(3.0, 4.0);
    println!("r.area(): {:.2}", r.area());
    println!("r.name(): {:?}", r.name());
    println!("r: {:?}", r);

    let serialized = serde_json::to_string(&r).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Rectangle = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    // Triangle
    let t = Triangle::new(3.0, 4.0);
    println!("t.area(): {:.2}", t.area());
    println!("t.name(): {:?}", t.name());
    println!("t: {:?}", t);

    let serialized = serde_json::to_string(&t).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Triangle = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    // trait object
    let r: Box<dyn Geometry<f64>> = Box::new(Rectangle::new(3.0, 4.0));
    let t: Box<dyn Geometry<f32>> = Box::new(Triangle::new(3.0, 4.0));
    println!("r: {:?}", r);
    println!("t: {:?}", t);

    // From trait
    println!("\n--- From trait ---");
    let r = Rectangle::from(3.0);
    println!("r: {:?}", r);
    let t = Triangle::from(3.0);
    println!("t: {:?}", t);
}
