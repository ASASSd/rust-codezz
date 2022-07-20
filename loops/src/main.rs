fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
    for number in (1..6).rev() {
        println!("the number is: {}", number);
    }
}
