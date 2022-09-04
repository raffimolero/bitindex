use bitindex::Bits;

fn main() {
    println!("don't try this at home");

    let mut flags = 0_u128;
    let mut b = Bits::from(&mut flags);
    b[3] = true;
    b[4] = true;
    println!("{}", b[3]);
    b[3] = true;
    drop(b);

    println!("{flags:b}");
}
