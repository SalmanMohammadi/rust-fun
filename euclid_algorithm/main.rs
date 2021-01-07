use std::io::{self, Write};

fn main() {
    println!("Euclid's algorithm.");

    let mut m = String::new();
    let mut n = String::new();
    print!("m:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut m)
        .expect("Failed to read line.");
    print!("n:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut n)
            .expect("Failed to read line.");

    let mut m: u32 = m.trim().parse()
        .expect("Type a number!");
    let mut n: u32 = n.trim().parse()
        .expect("Type a number!");

    if m < n {
        let x: u32 = n;
        n = m;
        m = x;
    }
    let mut r: u32 = m % n;
    while r != 0 {
        m = n;
        n = r;
        r = m % n;
    }
    println!("gcd:{}", n);
}
