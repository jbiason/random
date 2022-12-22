//! Testing ControlFlow

use std::ops::ControlFlow;

#[derive(Debug)]
enum ValueError {
    Odd,
    Zero,
}

fn main() {
    let invalid = [1, 3, 4, 0, 2, 4, 1];
    let all_valid = [2, 2, 2, 2, 4];

    let res_invalid = process(&invalid);
    println!("Invalid: {:?}", res_invalid);
}

fn process(values: &[u8]) -> Result<Vec<u8>, ValueError> {
    let result = values
        .iter()
        .map(|x| {
            if *x == 0 {
                ControlFlow::Break(Err(ValueError::Zero))
                // or `return Err(ValueError::Zero)`
            } else if *x % 2 != 0 {
                ControlFlow::Break(Err(ValueError::Odd))
                // or `return Err(ValueError::Odd)`
            } else {
                ControlFlow::Continue(*x * 2)
            }
        })
        .collect::<Vec<u8>>();
    Ok(result)
}
