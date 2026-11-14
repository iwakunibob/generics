use num_traits::{ToPrimitive};

fn pythagorean<T:ToPrimitive, U:ToPrimitive>(a: T, b: U) -> f64 {  // Float is a trait
    let a_f64: f64 = a.to_f64().unwrap();
    let b_f64: f64 = b.to_f64().unwrap();
    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}
fn main() {
    let a: u8 = 2;
    let b: i32 = 3;
    println!("A Triangle with sides {} x {} has a hypotenuse = {}", 
        a, b, pythagorean(a, b));
}
