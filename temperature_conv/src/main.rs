use std::io;

fn far2cels(temp: f64) -> f64 {
    (temp - 32.0)/1.8
}

fn main() {
    loop {
        println!("[?]   Enter temperature in Fahrenheit (or enter 15-digit Pi number to exit):");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("[!]  Read error");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[!]   Float parse error: found {}", temp);
                continue;
            }
        };
        if temp == 3.141592653589793 {
            println!("[bye] Exiting now.");
            break;
        } else {
            println!("[ok]  Temperature in Celsius is {}", far2cels(temp));
        }
    }
}
