#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn calculated_area(&self) -> u64 {
        let length_64: u64 = self.length.try_into().unwrap();
        let width_64: u64 = self.width.try_into().unwrap();
        length_64 * width_64
    }
}

fn area(rect1: &Rectangle) -> u64 {
    let length_64: u64 = rect1.length.try_into().unwrap();
    let width_64: u64 = rect1.width.try_into().unwrap();
    length_64 * width_64
}

fn main() {
    let rt1 = Rectangle {
        length: 36,
        width: 24,
    };
    let rt1 = dbg!(&rt1);
    println!("Area of film frame {:#?} is {}", rt1, rt1.calculated_area());
}
