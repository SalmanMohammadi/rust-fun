#![warn(clippy::all)]
use rand::{thread_rng, Rng};
mod structs;
use nannou::prelude::*;

struct Model {
    population: structs::Population
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let width = 1000.0;
    let height = 1000.0;
    app.new_window().size(width as u32, height as u32).view(view).build().unwrap();
    
    let target = (vec2(0.0,  250.0), 10.0);
    let mut obstacles = Vec::<(Vector2, f32, f32)>::new();

    // add some obstacles
    obstacles.push((vec2(0.0, 200.0), 200.0, 10.0));
    obstacles.push((vec2(-100.0, 225.0), 10.0, 50.0));
    obstacles.push((vec2(100.0, 225.0), 10.0, 50.0));

    // hacking window boundary collision - just a thin rect on each side
    obstacles.push((vec2(0.0, height/2.0), width, 3.0)); // top
    obstacles.push((vec2(0.0, -height/2.0), width, 3.0)); // bottom
    obstacles.push((vec2(width/2.0, 0.0), 3.0, height)); // right
    obstacles.push((vec2(-width/2.0, 0.0), 3.0, height)); // left
    let population = structs::Population::new(100, 600, target, obstacles);

    Model {
        population
    }
}

fn update(_app: &App, model: &mut Model, _update: Update){
    model.population.run();
}

fn view(app: &App,  model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    model.population.show(&draw);
    draw.to_frame(app, &frame).unwrap();
}

