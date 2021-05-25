use nannou::prelude::*;
use rand::seq::SliceRandom;
use nannou::color;

const WIN_W:u32 = 600;
const WIN_H:u32 = 900;
const SQUARE_W:f32 = 60.0;
const SQUARE_H:f32 = 80.0;
const NUM_X:i32 = 7;
const NUM_Y:i32 = 9;
const MAX_NUM_SQUARES:usize = 10;


fn main() {
    nannou::app(model).update(update).run();
}

struct Square {
    points: Vec<Vector2>
}

impl Square {
    pub fn new() -> Self {
        let w = SQUARE_W;
        let h = SQUARE_H;

        let mut points = vec!();
        let point1 = pt2((random_f32()) * -w, (random_f32()) * -h);
        points.push(point1);
        points.push(pt2((random_f32()) * w, (random_f32()) * -h));
        points.push(pt2((random_f32()) * w, (random_f32()) * h));
        points.push(pt2((random_f32()) * -w, (random_f32()) * h));
        points.push(point1);
        Square {
            points
        }
    }
}

struct SquareStructure {
    position: Vector2,
    collection: Vec<Square>,
    color: color::Rgb
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

        let collection = (0..MAX_NUM_SQUARES).into_iter()
            .map(|_| Square::new())
            .collect();

        SquareStructure {
            position,
            collection,
            color: SquareStructure::rando_color()
        }
    }

    // new
    pub fn set_points(&mut self) {
        self.collection = (0..MAX_NUM_SQUARES).into_iter()
            .map(|_| Square::new())
            .collect();
    }

    // new
    pub fn set_color(&mut self) {
        self.color = SquareStructure::rando_color();
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

struct Model {
    _window: window::Id,
    square_structures: Vec<SquareStructure>,
    current_num_squares: usize // new
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title("Structure de Quadrilatères (Square Structures)")
        .size(WIN_W, WIN_H)
        .mouse_released(mouse_released) // add this
        .mouse_wheel(mouse_wheel) // add this
        .view(view)
        .build()
        .unwrap();

    let square_structures = gen_structures();

    Model { _window,
             square_structures,
             current_num_squares: MAX_NUM_SQUARES } // new
}

// add this
fn mouse_released(_app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            for structure in model.square_structures.iter_mut() {
                structure.set_points();
            }
        },
        MouseButton::Right => {
            for structure in model.square_structures.iter_mut() {
                structure.set_color();
            }
        },
        _ => {}
    }
}

// add this
fn mouse_wheel(_app: &App, model: &mut Model, dt: MouseScrollDelta, _phase: TouchPhase) {
    match dt {
        MouseScrollDelta::LineDelta(_, y) if y > 0.0 => {
            if model.current_num_squares < MAX_NUM_SQUARES {
                model.current_num_squares += 1;
            }
        },
        MouseScrollDelta::LineDelta(_, y) if y < 0.0 => {
            if model.current_num_squares > 1 {
                model.current_num_squares -= 1;
            }
        },
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.88, 0.87, 0.85);

    for structure in model.square_structures.iter() {
        for square in structure.collection.iter().take(model.current_num_squares) { // change here
            draw.polyline()
                .xy(structure.position)
                .color(structure.color)
                .weight(3.0)
                .points(square.points.iter().cloned());
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
