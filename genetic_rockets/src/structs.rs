use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use nannou::prelude::*;
use nannou::draw::Draw;

pub struct Population {
    // pub mutation_rate: f32,
    pub rockets: Vec<Rocket>,
    pub population_size: usize
    // pub matingPool: Vec<Rocket>,
    // generations: u32
}

impl Population {
    pub fn new(population_size: usize) -> Population {
        let mut rng = thread_rng();
        let mut rockets = Vec::<Rocket>::new();
        for _ in 0..population_size{
            rockets.push(Rocket::new(&mut rng));
        }
        Population {
            rockets,
            population_size
        }
    }

    pub fn run(&mut self) {
        for i in 0..self.population_size{
            self.rockets[i].update();
        }
    }

    pub fn show(&self, draw: &Draw) {
        for i in 0..self.population_size{
            self.rockets[i].show(draw);
        }
    }

    pub fn fitness() {}
    pub fn selection() {}
    pub fn reproduction() {} 
}

pub struct Rocket { 
    // pub dna: DNA,

    // pub fitness: f32,
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    width: f32,
    height: f32
}

impl Rocket {
    pub fn new(rng: &mut ThreadRng) -> Rocket {
        let angle = rng.gen_range(0.0..PI*2.0);
        Rocket {
            position: vec2(0.0, 0.0),
            velocity: Vector2::from_angle(angle),
            acceleration: vec2(0.0, 0.0),
            width: 10.0,
            height: 50.0,
        }
    }

    pub fn show(&self, draw: &Draw) {
        let points = vec![pt2(self.width/2.0, -self.height/2.0),
                          pt2(-self.width/2.0, -self.height/2.0),  
                          pt2(-self.width/2.0, self.height/2.0), 
                          pt2(self.width/2.0, self.height/2.0)];
        draw.polygon()
            .stroke(WHITE)
            .stroke_weight(1.0)
            .points(points)
            .xy(self.position)
            .rotate(PI/2.0 + self.velocity.angle());
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }
   
    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }
}
//     pub fn new( mut rng: rand::rngs::ThreadRng) -> Rocket {
//         let dna: DNA = DNA::new(4, rng);
//         let max_speed = dna.genes[0];
//         let max_force = dna.genes[1];
//         let size = dna.genes[2];
//         let seperation_weight = dna.genes[3];
//         Vehicle {
//             dna: dna,
//             max_speed: max_speed,
//             max_force: max_force,
//             size: size,
//             seperation_weight: seperation_weight
//         }
//     }
// }

// struct DNA {
//     pub genes: Vec<(f32, f32)>
// }

// impl DNA {
//     pub fn new(lifetime: u32, max_force: f32, mut rng: rand::rngs::ThreadRng) -> DNA {
//         let mut genes = Vec::<(f32, f32)>::new();
//         for i in 0..lifetime {
//             let theta = rng.gen_range(0..360) as f32;
//             let force = rng.gen_range(0.0..max_force);
//             genes.push((theta.cos() * force, theta.sin() * force));
//         }
//         DNA {
//             genes: genes
//         }
//     }
// }
