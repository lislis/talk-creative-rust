use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    //let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(PLUM);

    // let pad = 25.0;
    // let win_p = win.pad(pad);
    // draw.rect()
    //     .xy(win_p.xy())
    //     .wh(win_p.wh())
    //     .color(rgba(0.3, 0.4, 0.7, 0.5));

    // let r = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);
    // draw.rect()
    //     .xy(r.xy())
    //     .wh(r.wh())
    //     .color(STEELBLUE);


    // let c = r.below(r).shift_y(-pad);
    // draw.ellipse()
    //     .xy(c.xy())
    //     .wh(c.wh())
    //     .color(STEELBLUE);

    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    let boundary = app.window_rect();

    // Store the radius of the circle we want to make.
    //let radius = 150.0;
    let radius = map_range(slowersine, -1.0, 1.0, 100.0, 300.0);

    let step = map_range(sine, -1.0, 1.0, 90, 30);

    // Map over an array of integers from 0 to 360 to represent the degrees in a circle.
    let points = (0..=360).step_by(step).map(|i| {
        // Convert each degree to radians.
        let radian = deg_to_rad(i as f32);
        // Get the sine of the radian to find the x co-ordinate of this point of the circle
        // and multiply it by the radius.
        let x = radian.sin() * radius;
        //let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
        // Do the same with cosine to find the y co-ordinate.
        let y = radian.cos() * radius;
        // Construct and return a point object with a color.
        (pt2(x,y), STEELBLUE)
    });
    // Create a polyline builder. Hot-tip: polyline is short-hand for a path that is
    // drawn via "stroke" tessellation rather than "fill" tessellation.
    draw.polygon()
        //.weight(3.0)
        .points_colored(points); // Submit our points.

    draw.to_frame(app, &frame).unwrap();
}
