use rand::Rng;


pub struct Vehicle { 
    pub dna: DNA,
    pub max_speed: f32,
    pub max_force: f32,
    pub size: f32,
    pub seperation_weight: f32
}

impl Vehicle {
    pub fn new( mut rng: rand::rngs::ThreadRng) -> Vehicle {
        let dna: DNA = DNA::new(4, rng);
        let max_speed = dna.genes[0];
        let max_force = dna.genes[1];
        let size = dna.genes[2];
        let seperation_weight = dna.genes[3];
        Vehicle {
            dna: dna,
            max_speed: max_speed,
            max_force: max_force,
            size: size,
            seperation_weight: seperation_weight
        }
    }
}

pub struct DNA {
    pub genes: Vec<f32>
}

impl DNA {
    pub fn new(num: u32, mut rng: rand::rngs::ThreadRng) -> DNA {
        let mut genes: Vec<f32> = Vec::new();
        for i in 0..num {
            genes.push(rng.gen_range(0.0..1.0));
        }
        DNA {
            genes: genes
        }
    }
}
