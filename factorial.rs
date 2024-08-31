fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else{
        n * factorial(n-1)
    }
}

fn main() {
    let result = factorial(7);
    println!("factorial of 7 is: {}", result);
}