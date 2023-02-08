fn main() {
    let evens = [2, 2, 2, 2, 2];
    let res = evens
        .iter()
        .map(|v| {
            if v % 2 == 0 {
                Ok(v.to_string())
            } else {
                Err("No".to_string())
            }
        })
        .collect::<Result<Vec<String>, String>>();
    println!("Evens: {:?}", res);

    let not_evens = [2, 2, 2, 2, 1, 2];
    let res = not_evens
        .iter()
        .map(|v| {
            if v % 2 == 0 {
                Ok(v.to_string())
            } else {
                Err("No".to_string())
            }
        })
        .map(|res| match res {
            Ok(v) => v,
            Err(e) => e,
        })
        .collect::<Vec<String>>();
    println!("Not evens: {:?}", res);
}
