fn modifies( x: &mut f64) -> f64{
    *x = 1.0;
    return *x;
}

fn main(){
    let mut res = 0.4;
    let value = modifies(&mut res);
    println!(" The value has been changed from {} -> {}", res, value)
}