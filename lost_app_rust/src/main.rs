use std::io;

fn is_array_correct(dat: [u8; 6]) -> bool {
    true
}
fn main() {
    print!("> ");
    let mut in_str = String::new();
    io::stdin().read_line(&mut in_str).expect("!Error!");
    println!("{}", in_str);
}
