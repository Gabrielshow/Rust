fn main() {
    let s1 = String::from("Hello world");
    let s2 = String::from("Citizens");

    let longest_string = find_longest_string(s1.as_str(), s2);
    println!("Longest string is {}", longest_string);
    longest_string 
}

fn  find_longest_string<'a>(string1: &'a str,string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

// Generic type parameters, trait bounds, and lifetimes together
