use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");
    println!("{}", args);
}
