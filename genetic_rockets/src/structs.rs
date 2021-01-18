use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use nannou::prelude::*;
use nannou::draw::Draw;
use nannou::color::*;

pub struct Population {
    // pub mutation_rate: f32,
    rockets: Vec<Rocket>,
    mating_pool: Vec<Rocket>,
    population_size: usize,
    lifetime: usize,
    count: usize,
    target: (Vector2, f32),
    rng: ThreadRng
    // pub matingPool: Vec<Rocket>,
    // generations: u32
}

impl Population {
    pub fn new(population_size: usize, lifetime: usize, window_rect: Rect) -> Population {
        let mut rng = thread_rng();
        let mut rockets = Vec::<Rocket>::new();
        let mut mating_pool = Vec::<Rocket>::new();
        for _ in 0..population_size{
            rockets.push(Rocket::new(&mut rng, lifetime));
        }
        Population {
            rockets,
            mating_pool,
            population_size, 
            lifetime,
            rng,
            count: 0,
            target: (vec2(0.0,  window_rect.y.end - 50.0), 10.0)
        }
    }

    pub fn run(&mut self) {
        for rocket in &mut self.rockets{
            rocket.update(self.count);
        }
        self.count += 1;
        if self.count == self.lifetime {
            self.evaluate();
            self.selection();
            self.count = 0;
        }
        
    }

    pub fn show(&self, draw: &Draw) {
        for rocket in &self.rockets{
            rocket.show(draw);
        }
        draw.ellipse().xy(self.target.0).radius(self.target.1);
    }

    fn evaluate(&mut self) {
        
        let mut max_fitness = 0.0;
        for rocket in &mut self.rockets{
            rocket.calculate_fitness(self.target.0);
            if rocket.fitness > max_fitness{
                max_fitness = rocket.fitness;
            }

        }

        for rocket in &mut self.rockets{
            rocket.fitness /= max_fitness;
        }

        self.mating_pool = Vec::<Rocket>::new();
        for i in 0..self.population_size{
            let n = self.rockets[i].fitness * 100.0;
            for _ in 0..n as usize{
                self.mating_pool.push(self.rockets[i].clone());
            }
        }
   

    }
    fn selection(&mut self) {
        let mut new_rockets = Vec::<Rocket>::new();
        for _ in 0..self.population_size {
            let parent_a = self.mating_pool[self.rng.gen_range(0..self.mating_pool.len())].clone().dna;
            let parent_b = self.mating_pool[self.rng.gen_range(0..self.mating_pool.len())].clone().dna;
            let child = parent_a.crossover(parent_b, &mut self.rng);
            new_rockets.push(Rocket::new_from_dna(child));
        }
        self.rockets = new_rockets;
    }
    
}

#[derive(Clone)]
struct Rocket { 

    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    dna: DNA,
    width: f32,
    height: f32,
    fitness: f32,
}

impl Rocket {
    fn new(rng: &mut ThreadRng, lifetime: usize) -> Rocket {
        Rocket {
            position: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            dna: DNA::new(lifetime, rng),
            width: 5.0,
            height: 25.0,
            fitness: 0.0
        }
    }

    fn new_from_dna(dna: DNA) -> Rocket{
        Rocket {
            position: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            dna,
            width: 5.0,
            height: 25.0,
            fitness: 0.0
        }
    }

    fn show(&self, draw: &Draw) {
        let points = vec![pt2(self.width/2.0, -self.height/2.0),
                          pt2(-self.width/2.0, -self.height/2.0),  
                          pt2(-self.width/2.0, self.height/2.0), 
                          pt2(self.width/2.0, self.height/2.0)];
        draw.polygon()
            .color(Alpha {color:WHITE, alpha: 0.5 })
            .points(points)
            .xy(self.position)
            .rotate(PI/2.0 + self.velocity.angle());
    }

    fn update(&mut self, count: usize) {
        self.apply_force(self.dna.genes[count]);
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }
   
    fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }

    fn calculate_fitness(&mut self, target: Vector2) {
        let distance_sqaured: f32 = (target.x - self.position.x).pow(2) + (target.y - self.position.y).pow(2);
        let distance = distance_sqaured.sqrt();
        self.fitness = 1.0/distance; // fix
    }

}

#[derive(Clone)]
struct DNA {
    genes: Vec<Vector2>
}

impl DNA {
    fn new(lifetime: usize,  rng: &mut ThreadRng) -> DNA {
        let mut genes = Vec::<Vector2>::new();
        for _ in 0..lifetime {
            genes.push(Vector2::from_angle(rng.gen_range(0.0..PI*2.0)) * 0.1);
        }
        DNA {
            genes
        }
    }

    fn crossover(&self, partner: DNA, rng: &mut ThreadRng) -> DNA {
        let mut genes = Vec::<Vector2>::new();
        let mid = rng.gen_range(0..self.genes.len()); //floor()?
        // println!("mid:{}", mid);
        for i in 0..self.genes.len(){
            if i > mid {
                genes.push(self.genes[i]);
            } else {
                genes.push(partner.genes[i]);
            }
        }
        DNA { genes }
    }

    
}
