use std::io::stdin;

const fn double(x: i32) -> i32 {
    x * 2
}

const FIVE: i32 = 5;
const TEN: i32 = double(FIVE);

fn main() {
    assert_eq!(5, FIVE);
    assert_eq!(10, double(FIVE));

    println!("Your const is {}", double(10));

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    println!("Your double: {}", double(s.trim().parse::<i32>().unwrap()));
}
