extern crate nannou;
use nannou::prelude::*;
use nannou::draw::Drawing;
use nannou::draw::primitive::Rect;
use nannou::geom::point::Point2;
use std::io::{self, Write};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn create_rect(draw: &Draw, x: f32, y: f32, w: f32, h: f32) -> Vec<Point2>{
    let blc = pt2(x-(w/2.0), y-(h/2.0));
    let brc = pt2(x+(w/2.0), y-(h/2.0));
    let tlc = pt2(x-(w/2.0), y+(h/2.0));
    let trc = pt2(x+(w/2.0), y+(h/2.0));
    return vec![tlc, trc, brc, blc];
}
fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    draw.background().color(BLACK);
    
    let mut rects = Vec::new();

    let win = _app.window_rect();
   
    // add window boundaries for entire scene raycast
    //rects.push(vec![win.top_left(), win.top_right(), win.bottom_right(), win.bottom_left()]);

    
    // draw static environment
    rects.push(create_rect(&draw, 100.0, 100.0, 100.0, 100.0));
    rects.push(create_rect(&draw, -100.0, 200.0, 100.0, 100.0));
    rects.push(create_rect(&draw, 0.0, -100.0, 100.0, 100.0));
    rects.push(create_rect(&draw, -200.0, -100.0, 100.0, 100.0));
    
    // draw the mouse
    let start_point = pt2(_app.mouse.x, _app.mouse.y);


    for i in 0..4 {
        for (x, y) in (1..4).enumerate(){
            print!("x: {}, y:{}\n", x, y%4);
            draw.tri()
                .color(rgba(150.0, 255.0, 0.0, 0.3))
                .points(rects[i][x], rects[i][y%4],  start_point);
            
        }
        //for j in 0..4{
        //    draw.line()
        //        .start(start_point)
        //        .end(rects[i][j])
        //        .weight(1.0)
        //        .color(WHITE);   
        //} 

        // let points = (0..4).map(|x| {
        //     (rects[i][x], RED)
        // });
        // draw.polygon()
        //     .points_colored(points);
    }
    //for i in 1..5 {
      //  let points = (0..4).map(|x| {
       //     (rects[i][x], RED)
       // });
       // draw.polygon()
        //    .points_colored(points);
    //}

    draw.ellipse()
        .xy(start_point)
        .radius(5.0)
        .color(RED);
    // draw.path().fill().color(RED).events()
    
    draw.to_frame(_app, &frame).unwrap();
}
