fn main() {
    operators();
}

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4; // + - * /
    println!("2 + 3 * 4 = {}", a);
    a += 5;
    a -= 1;
    println!("a is {}", a);
    println!("a % 3 is {}", a % 17);

    let power = i32::pow(15, 2);
    println!("15 squared is {}", power);

    println!("Pi is {}", std::f64::consts::PI);

    //bitwise operators
    let c = 1 | 2; // | OR operator & AND ^ XOR

}
