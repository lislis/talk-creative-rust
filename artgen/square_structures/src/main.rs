use nannou::prelude::*;
use rand::seq::SliceRandom;
use nannou::color;

const QUAD_W:f32 = 60.0;
const QUAD_H:f32 = 80.0;
const NUM_QUAD_X:i32 = 7;
const NUM_QUAD_Y:i32 = 9;
const NUM_QUAD_SCRIBBLES:i32 = 7;
const WIN_W:u32 = 600;
const WIN_H:u32 = 900;

fn main() {
    nannou::app(model).update(update).run();
}

fn gen_points(w:f32, h:f32) -> Vec<Vector2> {
    let mut points = vec!();
    let point1 = pt2((random_f32()) * -w, (random_f32()) * -h);
    points.push(point1);
    points.push(pt2((random_f32()) * w, (random_f32()) * -h));
    points.push(pt2((random_f32()) * w, (random_f32()) * h));
    points.push(pt2((random_f32()) * -w, (random_f32()) * h));
    points.push(point1);
    points
}

fn gen_quads() -> Vec<Quad>{
    let mut quads = vec!();
    for i in 0..NUM_QUAD_X {
        for j in 0..NUM_QUAD_Y {
            let c = Quad::rando_color();
            for _num in 0..NUM_QUAD_SCRIBBLES {
                let mut q = Quad::new(i as f32, j as f32,
                                      WIN_W as f32, WIN_H as f32,
                                      QUAD_W, QUAD_H);
                q.set_color(c);
                quads.push(q);
            }
        }
    }
    quads
}

#[derive(Debug)]
struct Quad {
    pub position: Vector2,
    pub points: Vec<Vector2>,
    color: color::Rgb
}

impl Quad {
    pub fn new(i_x: f32, i_y: f32,
               max_x: f32, max_y: f32,
               w: f32, h: f32) -> Self {
        let x = i_x * w;
        let y = i_y * h;
        let x = x - (max_x * 0.3);
        let y = y - (max_y * 0.35);

        Quad {
            position: pt2(x, y),
            points: gen_points(w, h),
            color: Quad::rando_color()
        }
    }
    pub fn set_points(&mut self, w: f32, h: f32) {
        self.points = gen_points(w, h);
    }
    pub fn set_color(&mut self, color: color::Rgb) {
        self.color = color;
    }
    pub fn rando_color() -> color::Rgb {
        let colors = vec!(
            rgb(0.38, 0.68, 0.67), // green
            rgb(0.1, 0.43, 0.65), // blue
            rgb(0.65, 0.54, 0.68), // purple
            rgb(0.92, 0.54, 0.38), // orange
            rgb(0.24, 0.24, 0.24) // black
        );
        let color = colors.choose(&mut rand::thread_rng()).unwrap();
        *color
    }
}

struct Model {
    quads: Vec<Quad>
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Structure de Quadrilatères (Square Structures)")
        .size(WIN_W, WIN_H)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .view(view)
        .build()
        .unwrap();

    //app.set_loop_mode(LoopMode::loop_once());
    app.set_loop_mode(LoopMode::Wait);
    //app.set_loop_mode(LoopMode::rate_fps(4.0));

    Model { quads: gen_quads() }
}

fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    match _button {
        MouseButton::Left => {
            println!("LEFT");

            for quad in model.quads.iter_mut() {
                quad.set_points(QUAD_W, QUAD_H);
            }
        },
        MouseButton::Right => {
            println!("RIGHT");
            model.quads = gen_quads();
        },
        _ => {}
    }
}

fn mouse_wheel(_app: &App, _model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {
    match _dt {
        MouseScrollDelta::LineDelta(_, y) if y > 0.0 => {
            println!("scroll up");
        },
        MouseScrollDelta::LineDelta(_, y) if y < 0.0 => {
            println!("scroll down");
        },
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.88, 0.87, 0.85);

    // map_range(app.time.sin(), -1.0, 1.0, 1.0, 3.0)

    for quad in model.quads.iter() {
        draw.polyline()
            .xy(quad.position)
            .color(quad.color)
            .weight(3.0)
            .points(quad.points.iter().cloned());
    }

    draw.to_frame(app, &frame).unwrap();
}
