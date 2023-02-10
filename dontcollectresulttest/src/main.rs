fn is_even(val: u8) -> Result<u8, u8> {
    if val % 2 == 0 {
        Ok(val)
    } else {
        Err(val)
    }
}
fn main() {
    let values = [2u8, 2, 2, 2, 2, 1];
    println!(
        "Result<Vec>: {:?}",
        values
            .iter()
            .map(|x| is_even(*x))
            .collect::<Result<Vec<u8>, u8>>()
    );

    print!(
        "Vec<Result>: {:?}",
        values
            .iter()
            .map(|x| is_even(*x))
            .collect::<Vec<Result<u8, u8>>>()
    );
}
