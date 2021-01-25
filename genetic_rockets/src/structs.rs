use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use nannou::prelude::*;
use nannou::draw::Draw;
use nannou::color::*;

fn euclidean_distance(vec1: Vector2, vec2: Vector2) -> f32 {
    let distance_squared: f32 = (vec1.x - vec2.x).pow(2) + (vec1.y -vec2.y).pow(2);
    distance_squared.sqrt()
}

// checks if pos intersects wiith the rect at ((x, y), w, h)
fn rect_intersect(pos: Vector2, (center, width, height): (Vector2, f32, f32)) -> bool {
    pos.x >= center.x - width/2.0 && pos.x <= center.x + width/2.0 
    && pos.y >= center.y - height/2.0 && pos.y <= center.y + height/2.0
}

pub struct Population {
    rockets: Vec<Rocket>,
    mating_pool: Vec<Rocket>,
    population_size: usize,
    lifetime: usize,
    count: usize,
    target: (Vector2, f32),
    rng: ThreadRng,
    obstacles: Vec<(Vector2, f32, f32)>
}

impl Population {
    pub fn new(population_size: usize, lifetime: usize, target: (Vector2, f32), obstacles: Vec<(Vector2, f32, f32)>) -> Population {
        let mut rng = thread_rng();
        let mut rockets = Vec::<Rocket>::new();
        for _ in 0..population_size{
            rockets.push(Rocket::new(&mut rng, lifetime));
        }
        Population {
            rockets,
            mating_pool: Vec::<Rocket>::new(),
            population_size, 
            lifetime,
            rng,
            count: 0,
            target,
            obstacles
        }
    }

    pub fn run(&mut self) {
        for rocket in &mut self.rockets{
            rocket.update(self.count, self.target);
            rocket.detect_collision(&mut self.obstacles);
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
        for obstacle in &self.obstacles {
            draw.rect().xy(obstacle.0).w(obstacle.1).h(obstacle.2);
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
        println!("mating_pool.len(): {}", self.mating_pool.len());
        for _ in 0..self.population_size {
            let parent_a = self.mating_pool[self.rng.gen_range(0..self.mating_pool.len())].clone().dna;
            let parent_b = self.mating_pool[self.rng.gen_range(0..self.mating_pool.len())].clone().dna;
            let mut child = parent_a.crossover(parent_b, &mut self.rng);
            child.mutation(&mut self.rng);
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
    completed: bool,
    crashed: bool,
}

impl Rocket {
    fn new(rng: &mut ThreadRng, lifetime: usize) -> Rocket {
        Rocket {
            position: vec2(0.0, -450.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            dna: DNA::new(lifetime, rng),
            width: 5.0,
            height: 25.0,
            fitness: 0.0,
            completed: false,
            crashed: false
        }
    }

    fn new_from_dna(dna: DNA) -> Rocket{
        Rocket {
            position: vec2(0.0, -450.0),
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(0.0, 0.0),
            dna,
            width: 5.0,
            height: 25.0,
            fitness: 0.0,
            completed:  false,
            crashed: false
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

    fn update(&mut self, count: usize, target: (Vector2, f32)) {
        let distance = euclidean_distance(self.position, target.0);
        if distance < target.1 {   
            self.completed = true;
            self.position = target.0;
        }
        if !self.completed && !self.crashed {
            self.apply_force(self.dna.genes[count]);
            self.velocity += self.acceleration;
            self.position += self.velocity;
            self.acceleration *= 0.0;
        }
    }
   
    fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }

    fn calculate_fitness(&mut self, target: Vector2) {
        let distance: f32 = euclidean_distance(target, self.position);
        //
            // Map the sine wave functions to ranges between the boundaries of the window
    // let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    // let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());
        self.fitness = 1.0 /(distance + 1.0.pow(-16) as f32); // fix
        if self.completed {
            self.fitness *= 10.0;
        }
        if self.crashed {
            self.fitness *= 0.1;
        }

    }

    fn detect_collision(&mut self, obstacles: &mut  Vec<(Vector2, f32, f32)>) {
        self.crashed = obstacles.iter().any(|rect| rect_intersect(self.position, *rect));
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
            genes.push(Vector2::from_angle(rng.gen_range(0.0..2.0*PI)) * 0.2);
        }
        DNA {
            genes
        }
    }

    fn crossover(&self, partner: DNA, rng: &mut ThreadRng) -> DNA {
        let mut genes = Vec::<Vector2>::new();
        let mid = rng.gen_range(0..self.genes.len());
        for i in 0..self.genes.len(){
            if i > mid {
                genes.push(self.genes[i]);
            } else {
                genes.push(partner.genes[i]);
            }
        }
        DNA { genes }
    }

    fn mutation(&mut self, rng: &mut ThreadRng) {
        for i in 0..self.genes.len() {
            if rng.gen_range(0.0..1.0) < 0.01 {
                self.genes[i] = Vector2::from_angle(rng.gen_range(0.0..2.0*PI)) * 0.2
            }
        }
    }
    
}
