mod structs;

fn main() {
    let mut rng = rand::thread_rng();
    let dna:structs::DNA = structs::DNA::new(15, rng);
    for code in dna.genes.into_iter() {
        println!("{}", code);
    }
}
