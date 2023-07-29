use std::io::{stdin, stdout, Write};

use bitindex::Bits;

fn get_index() -> u8 {
    let mut buf = String::new();
    for _ in 0..3 {
        println!("Enter an index.");
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        let Ok(num) = buf.trim().parse::<u8>() else {
            println!("Must be a positive natural number.");
            continue;
        };

        if num >= 128 {
            println!("Too large. Must be from 0 to 127.");
            continue;
        }
        return num;
    }
    println!("3 wrong answers. exiting...");
    return 255;
}

fn main() {
    println!("don't try this at home");

    let mut flags = 0_u128;
    loop {
        let mut b = Bits::from(&mut flags);
        let index = get_index();
        // forgive me for the sentinel value
        if index == 255 {
            break;
        }
        b[index] = !b[index];
        println!("{}", b[index]);

        for bit in b {
            print!("{}", if bit { '#' } else { '.' });
        }
        println!();
    }
}
