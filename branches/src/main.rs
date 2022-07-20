fn main() {
    let number = 0;    //integer condition (!)
    let number = if number != 0 { true } else { false };
    if number {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}