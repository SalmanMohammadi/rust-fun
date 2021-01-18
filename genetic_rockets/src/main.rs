#![warn(clippy::all)]
use rand::{thread_rng, Rng};
mod structs;
use nannou::prelude::*;

struct Model {
    population: structs::Population
}

fn main() {
    // let dna:structs::DNA = structs::DNA::new(15, rng);
    // for code in dna.genes.into_iter() {
    //     println!("{}", code);
    // }
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let lifetime: u32 = 500;
    let mut life_counter: u32=  0;
    let mutation_rate = 0.01;
    let population_size: u32 = 1;
    // let mut rocket = structs::Rocket::new();
    app.new_window().size(1000, 1000).view(view).build().unwrap();
    let mut population = structs::Population::new(25, 300, app.window_rect());

    println!("{}, {}", app.window_rect().xy().x, app.window_rect().xy().y);
    Model {
        population
    }
}

fn update(_app: &App, model: &mut Model, _update: Update){
    // model.rocket.update();
    model.population.run();
}

fn view(app: &App,  model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.population.show(&draw);

    
    draw.to_frame(app, &frame).unwrap();
}

