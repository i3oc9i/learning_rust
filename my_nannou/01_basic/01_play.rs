use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

fn model(_: &App) -> Model {
    let x = 100.0;
    let y = 100.0;
    let x_speed = 2.5;
    let y_speed = 2.0;

    Model {
        x,
        y,
        x_speed,
        y_speed,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.x = model.x + model.x_speed;
    model.y = model.y + model.y_speed;

    let win_rect = app.window_rect();

    if (model.x > win_rect.right()) || (model.x < win_rect.left()) {
        model.x_speed = model.x_speed * -1.0;
    }
    if (model.y > win_rect.top()) || (model.y < win_rect.bottom()) {
        model.y_speed = model.y_speed * -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(ORANGE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(50.0, 50.0)
        .rgba(0.0, 0.0, 1.0, 1.0)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
