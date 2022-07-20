fn main() {
    println!("Hello, world!");
    greeting("Ilya".to_string());

    let x1: u8 = 255;
    let y1: u8 = 255;
    println!("Sum of {} and {} is {}", x1, y1, sum(x1, y1));
}

fn greeting(x: String) {
    println!("Nice to meet you... {}.", x);
}

fn sum(x: u8, y: u8) -> u16 {
    let x: u16 = x.try_into().unwrap();
    let y: u16 = y.try_into().unwrap();
    x + y
}
