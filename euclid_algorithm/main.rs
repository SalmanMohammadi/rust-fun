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

    let m: u32 = m.trim().parse()
        .expect("Type a number!");
    let n: u32 = n.trim().parse()
        .expect("Type a number!");

    let  mut r = m & n;
    while r != 0 {
        
    }
    println!("{}", r);
}
