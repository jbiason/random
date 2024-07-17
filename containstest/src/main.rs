const INVALID: &'static str =  " ()[]/\"\\";

fn invalid(check: &str) -> bool {
    check.chars().any(|char| INVALID.contains(char))
}

fn main() {
    let str = "thisisfine";
    println!("{} - {}", str, invalid(str));

    let str = "this is not fine";
    println!("{} - {}", str, invalid(str));
}
