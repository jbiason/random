fn main() {
    let data = [0b1000_0001u8, 0b0000_0010, 0b1000_0011];
    let mut main_iter = data.iter();
    let mut simple_iter = main_iter.take_while(|x| *x & 0b1000_000 != 0b1000_0000);
    println!("{:?}", simple_iter);

    let mut next_iter = main_iter.take_while(|x| *x & 0b1000_000 != 0b1000_0000);
    println!("{:?}", next_iter);
}
