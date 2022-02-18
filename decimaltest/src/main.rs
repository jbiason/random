use rust_decimal::Decimal;

fn main() {
    let initial = Decimal::new(202, 2);
    println!("202, 2 = {}", initial.to_string());

    // let negative = Decimal::new(202, -2);
    // println!("202, -2 = {}", negative.to_string());

    let value = Decimal::new(2345, 2);
    let inc = value + Decimal::new(5, 0);
    let dec = value - Decimal::new(5, 0);

    println!("Inc = {}; Dec = {}", inc.to_string(), dec.to_string())
}
