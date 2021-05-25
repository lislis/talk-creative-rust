use nannou::prelude::*;
use rand::seq::SliceRandom; // new
use nannou::color; // add this

const WIN_W:u32 = 600;
const WIN_H:u32 = 900;
const SQUARE_W:f32 = 60.0;
const SQUARE_H:f32 = 80.0;
const NUM_X:i32 = 7;
const NUM_Y:i32 = 9;


fn main() {
    nannou::app(model).update(update).run();
}

struct SquareStructure {
    position: Vector2,
    points: Vec<Vector2>,
    color: color::Rgb // add this
}

impl SquareStructure {
    pub fn new(i_x: f32, i_y: f32) -> Self {
        let w = SQUARE_W;
        let h = SQUARE_H;
        let max_x = WIN_W as f32;
        let max_y = WIN_H as f32;

        let x = i_x * w;
        let y = i_y * h;
        let x = x - (max_x * 0.3);
        let y = y - (max_y * 0.35);

        let position = vec2(x, y);

        let mut points = vec!();
        let point1 = pt2((random_f32()) * -w, (random_f32()) * -h);
        points.push(point1);
        points.push(pt2((random_f32()) * w, (random_f32()) * -h));
        points.push(pt2((random_f32()) * w, (random_f32()) * h));
        points.push(pt2((random_f32()) * -w, (random_f32()) * h));
        points.push(point1);

        SquareStructure {
            position,
            points,
            color: SquareStructure::rando_color() // new
        }
    }

    // add this
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
    _window: window::Id,
    square_structures: Vec<SquareStructure>
}

fn gen_structures() -> Vec<SquareStructure> {
    let mut sqrs = vec!();
    for i in 0..NUM_X {
        for j in 0..NUM_Y {
            let q = SquareStructure::new(i as f32, j as f32);
            sqrs.push(q);
        }
    }
    sqrs
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title("Structure de Quadrilatères (Square Structures)")
        .size(WIN_W, WIN_H)
        .view(view)
        .build()
        .unwrap();

    let square_structures = gen_structures();

    Model { _window,
             square_structures }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.88, 0.87, 0.85);

    for structure in model.square_structures.iter() {
        draw.polyline()
            .xy(structure.position)
            .color(structure.color) // boom
            .weight(3.0)
            .points(structure.points.iter().cloned());
    }

    draw.to_frame(app, &frame).unwrap();
}
