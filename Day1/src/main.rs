use std::env;

/// Problem 1
/// Input: in a list of names.
/// If the name starts with w, print Hello [name]
fn main() {
    let args: Vec<String> = env::args().collect();
    args.iter()
        .filter(|arg| arg.to_ascii_lowercase().chars().nth(0).unwrap().eq(&'w'))
        .for_each(|arg| println!("Hello, {} !", arg));
}
