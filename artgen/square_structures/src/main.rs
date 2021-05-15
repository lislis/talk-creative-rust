use nannou::prelude::*;
use rand::seq::SliceRandom;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("Structure de Quadrilatères (Square Structures)")
        .size(600, 900)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .view(view)
        .build()
        .unwrap();

    app.set_loop_mode(LoopMode::loop_once());
    //app.set_loop_mode(LoopMode::Wait);
    Model { _window }
}

fn mouse_released(app: &App, _model: &mut Model, _button: MouseButton) {
    match _button {
        MouseButton::Left => {
            println!("LEFT");
        },
        MouseButton::Right => {
            println!("RIGHT");
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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.88, 0.87, 0.85);

    let colors = vec!(
        srgba(0.38, 0.68, 0.67, 0.95), // green
        srgba(0.1, 0.43, 0.65, 0.95), // blue
        srgba(0.65, 0.54, 0.68, 0.95), // purple
        srgba(0.92, 0.54, 0.38, 0.95), // orange
        srgba(0.24, 0.24, 0.24, 0.95) // black
    );

    let win = app.window_rect();
    let d = 60.0;
    let d2 = 80.0;

    for i in 0..7 {
        for j in 0..9 {
            let x = i as f32 * (d * 1.0);
            let y = j as f32 * (d2 * 1.0);
            let x = x - (win.right() / 2.0 + (d * 0.5));
            let y = y + (win.bottom() / 2.0 - (d2 * 1.0));

            let color = colors.choose(&mut rand::thread_rng()).unwrap().color;

            for _num in 0..7 {
                let point1 = pt2(random_f32() * -1.0 * d, random_f32() * -1.0 * d2);
                let point2 = pt2(random_f32() * d, random_f32() * -1.0 * d2) ;
                let point3 = pt2(random_f32() * d, random_f32() * d);
                let point4 = pt2(random_f32() * -1.0 * d, random_f32() * d2);

                draw.polyline()
                    .x_y(x, y)
                    .weight(3.0)
                    .points_colored(vec!((point1, color),
                                         (point2, color),
                                         (point3, color),
                                         (point4, color),
                                         (point1, color)));
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
