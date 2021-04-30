use point::Point;
fn main() {
    let x = Point { x: 10, y: 20 };
    let y = Point { x: 30, y: 40 };
    let z = x + y;
    println!("Hello {:?}", z);
}
