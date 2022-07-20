fn fibonacci_nextnum(n1: u128, n2: u128) -> u128 {
    n1 + n2
}

fn main() {
    const NUMLINE_SIZE: usize = 187;
    let mut fib_nl: [u128; NUMLINE_SIZE] = [0; NUMLINE_SIZE];
    fib_nl[1] = 1;
    for i in 2..NUMLINE_SIZE {
        fib_nl[i] = fibonacci_nextnum(fib_nl[i-2], fib_nl[i-1]);
    }
    println!("Fibonacci numbers: ");
    for i in 0..(NUMLINE_SIZE - 1) {
        print!("{}, ", fib_nl[i]);
    }
    println!("{}", fib_nl[NUMLINE_SIZE - 1]);
}
