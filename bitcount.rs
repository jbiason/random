fn count(bits: usize) -> u32 {
    let num_bits = usize::BITS;
    num_bits as u32 - bits.leading_zeros() + 1
}

fn main() {
    println!("{}", count(0b111));
    println!("{}", count(0b1111_1111));
    println!("{}", count(64)); // 1000000
    println!("{}", 1 >> 1);
}
