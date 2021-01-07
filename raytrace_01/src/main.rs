use nannou::draw::primitive::Rect;
use nannou::draw::Drawing;
use nannou::geom::point::Point2;
use nannou::prelude::*;
use std::io::{self, Write};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn create_rect(x: f32, y: f32, w: f32, h: f32) -> Vec<Point2> {
    let blc = pt2(x - (w / 2.0), y - (h / 2.0));
    let brc = pt2(x + (w / 2.0), y - (h / 2.0));
    let tlc = pt2(x - (w / 2.0), y + (h / 2.0));
    let trc = pt2(x + (w / 2.0), y + (h / 2.0));
    return vec![blc, brc, trc, tlc, blc];
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);
    // draw some rects

    let mut rects = Vec::new();
    rects.push(create_rect(100.0, 100.0, 100.0, 100.0));
    rects.push(create_rect(-100.0, 200.0, 100.0, 100.0));

    for rect in rects {
        let points = rect.into_iter().map(|point| (point, STEELBLUE));
        draw.polyline().weight(3.0).points_colored(points);
    }
    let mouse = pt2(_app.mouse.x, _app.mouse.y);
    let radius = 50.0;
    // cast some rays
    for i in (0..361).step_by(10) {
        let i = i as f32;
        let rad = i * (std::f32::consts::PI / 180.0);
        let end = pt2(radius * rad.sin(), radius * rad.cos()) + mouse;
        draw.line().start(mouse).end(end).color(WHITE);
    }
    draw.ellipse().xy(mouse).radius(5.0).color(RED);
    draw.to_frame(_app, &frame).unwrap();
}
