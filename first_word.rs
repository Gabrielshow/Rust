fn first_word(str: &str) -> &str {
    let bytes = s.as_bytes()

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] 
}

fn main( ) {
    let mut s = String::from("hello world");

    let word = first_word(&s); //immutable borrow here
    // s.clear(); // this implies mutable borrow
}