fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn main() {
    let absolute = abs(-8.0);
    println!("The absolute value of -8 is : {}", absolute)
}