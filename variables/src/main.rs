use std::io;
use std::cmp::Ordering;

fn main() {
    let a: [u8; 16] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];

    loop {
        println!("Please enter an array index.          [?]");
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("Failed to read line                 [!]");
        let index: isize = match index.trim().parse() {
            Ok(res) => res,
            Err(_) => {
                println!("Index is not a number                 [!]");
                continue;
            }
        };
        let size_n: isize = 0;
        let size_t: isize = (a.len() - 1).try_into().unwrap();
        match index.cmp(&size_t) {
            Ordering::Greater => {
                println!("Index is out of bounds (Too high)     [!]");
                continue;
            },
            Ordering::Equal => {
                println!("Index is correct                      [ok]");
            },
            Ordering::Less => {
                match index.cmp(&size_n) {
                    Ordering::Greater => {
                        println!("Index is correct                      [ok]");
                    },
                    Ordering::Equal => {
                        println!("Index is correct                      [ok]");
                    },
                    Ordering::Less => {
                        println!("Index is out of bounds (Too low)      [!]");
                        continue;
                    }
                }
            }
        }
        let index: usize = index.try_into().unwrap();
        let element = a[index];
        println!("The value of the element at index {} is: {}", index, element);
    }
}