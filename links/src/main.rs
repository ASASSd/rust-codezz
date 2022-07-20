fn first_word(s: &str) -> &str {
    let s_bytes = s.as_bytes();

    for (i, &item) in s_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}

fn main() {
    let s1 = String::from("CROC is life");
    let s2 = first_word(&s1);
    // s1.clear();
    println!("First word in {} is {}", s1, s2);

}