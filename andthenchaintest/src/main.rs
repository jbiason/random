fn main() {
    let value: Result<u32, u32> = Err(20)
        .map_err(|_: u32| 30)
        .and_then(|_old: u32| Err(40));

    // Note: Order does not affect these results; even if `.map_err()` comes before `.and_then()`,
    // when `.and_then()` returns an error, it is captured by `.map_err()` again.

    println!("{:?}", value);
}
