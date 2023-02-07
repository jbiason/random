fn main() {
    let all_evens = [2u8, 4, 6, 8];
    let result: Result<Vec<_>, _> = all_evens
        .iter()
        .map(|value| if value % 2 == 0 { Some(value) } else { None })
        .map(|opt| opt.ok_or(format!("Not good")))
        .collect();
    println!("All Evens: {:?}", result);

    let not_evens = [2, 2, 2, 2, 2, 1];
    let result: Result<Vec<_>, _> = not_evens
        .iter()
        .map(|value| if value % 2 == 0 { Some(value) } else { None })
        .map(|opt| opt.ok_or(format!("Not good")))
        .collect();
    println!("Not Evens: {:?}", result);
}
