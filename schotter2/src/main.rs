use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const ROWS: u32 = 22;
const COLS: u32 = 12;
const SIZE: u32 = 30;
const LINE_WIDTH: f32 = 0.06;
const MARGIN: u32 = 35;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    random_seed: u64,
    disp_adj: f32,
    rot_adj: f32,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::wait());
    let _window = app.new_window()
                .title("Schotter2")
                .size(WIDTH, HEIGHT)
                .view(view)
                .key_pressed(key_pressed)
                .build()
                .unwrap();

    let random_seed = random_range(0, 1000000);
    let disp_adj = 1.0;
    let rot_adj = 1.0;
    Model {
        random_seed,
        disp_adj,
        rot_adj,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw.scale(SIZE as f32)
                    .scale_y(-1.0)
                    .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    let mut rng = StdRng::seed_from_u64(model.random_seed);

    draw.background().color(SNOW);

    for y in 0..ROWS {
        for x in 0..COLS {
            let factor = y as f32 / ROWS as f32;
            let disp_factor = factor * model.disp_adj;
            let rot_factor = factor * model.rot_adj;
            let x_offset = disp_factor * rng.gen_range(-0.5, 0.5);
            let y_offset = disp_factor * rng.gen_range(-0.5, 0.5);
            let rotation = rot_factor * rng.gen_range(-PI / 4.0, PI / 4.0);
            gdraw.rect()
                .no_fill()
                .stroke(BLACK)
                .stroke_weight(LINE_WIDTH)
                .w_h(1.0, 1.0)
                .x_y(x as f32 + x_offset, y as f32 + y_offset)
                .rotate(rotation)
                ;
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.random_seed = random_range(0, 1000000);
        }
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _other_key => {}
    }

}
