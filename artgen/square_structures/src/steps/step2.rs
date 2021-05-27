use nannou::prelude::*;

// add these
const WIN_W:u32 = 600;
const WIN_H:u32 = 900;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    points: Vec<Vector2> // add this
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title("Structure de Quadrilatères (Square Structures)") // add this
        .size(WIN_W, WIN_H) // and this
        .view(view)
        .build()
        .unwrap();

    // make a random rectangle
    let w = 100.0;
    let h = 100.0;

    let mut points = vec!();
    let point1 = pt2((random_f32()) * -w, (random_f32()) * -h);
    points.push(point1);
    points.push(pt2((random_f32()) * w, (random_f32()) * -h));
    points.push(pt2((random_f32()) * w, (random_f32()) * h));
    points.push(pt2((random_f32()) * -w, (random_f32()) * h));
    points.push(point1);

    Model { _window,
             points }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    // draw the rectangle
    draw.polyline()
        .color(STEELBLUE)
        .weight(3.0)
        .points(model.points.clone());

    draw.to_frame(app, &frame).unwrap();
}
