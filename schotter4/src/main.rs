use nannou::prelude::*;
use std::fs;
use std::io::ErrorKind;

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

struct Stone {
    x: f32,
    y: f32,
    x_offset: f32,
    y_offset: f32,
    rotation: f32,
    x_velocity: f32,
    y_velocity: f32,
    rot_velocity: f32,
    cycles: u32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        let x_offset = 0.0;
        let y_offset = 0.0;
        let rotation = 0.0;
        let x_velocity = 0.0;
        let y_velocity = 0.0;
        let rot_velocity = 0.0;
        let cycles = 0;
        Stone {
            x,
            y,
            x_offset,
            y_offset,
            rotation,
            x_velocity,
            y_velocity,
            rot_velocity,
            cycles,
        }
    }
}

struct Model {
    ui_window: Entity,
    frames_dir: String,
    cur_frame: u32,
    recording: bool,
    disp_adj: f32,
    rot_adj: f32,
    motion: f32,
    gravel: Vec<Stone>,
}

fn model(app: &App) -> Model {
    let ui_window = app.new_window()
                .always_on_top(true)
                .title(app.exe_name().unwrap() + " controls")
                .size(280, 130)
                .view(ui_view)
                .key_pressed(key_pressed)
                .build();

    let _window = app.new_window()
                .primary()
                .title(app.exe_name().unwrap())
                .size(WIDTH, HEIGHT)
                .view(view)
                .key_pressed(key_pressed)
                .build();

    let frames_dir = app.exe_name().unwrap() + "_frames";
    let recording = false;
    let cur_frame = 0;

    let disp_adj = 1.0;
    let rot_adj = 1.0;
    let motion = 0.5;

    let mut gravel = Vec::new();
    for y in 0..ROWS {
        for x in 0..COLS {
            let stone = Stone::new(x as f32, y as f32);
            gravel.push(stone);
        }
    }

    Model {
        ui_window,
        frames_dir,
        recording,
        cur_frame,
        disp_adj,
        rot_adj,
        motion,
        gravel,
    }
}

fn update(app: &App, model: &mut Model) {
    if app.window_count() < 2 {
        app.quit();
    }
    update_ui(app, model);
    for stone in &mut model.gravel {
        if stone.cycles == 0 {
            if random_f32() > model.motion {
                stone.x_velocity = 0.0;
                stone.y_velocity = 0.0;
                stone.rot_velocity = 0.0;
                stone.cycles = random_range(50, 300);
            } else {
                let factor = stone.y / ROWS as f32;
                let disp_factor = factor * model.disp_adj;
                let rot_factor = factor * model.rot_adj;
                let new_x = disp_factor * random_range(-0.5, 0.5);
                let new_y = disp_factor * random_range(-0.5, 0.5);
                let new_rot = rot_factor * random_range(-PI / 4.0, PI / 4.0);
                let new_cycles = random_range(50, 300);
                stone.x_velocity = (new_x - stone.x_offset) / new_cycles as f32;
                stone.y_velocity = (new_y - stone.y_offset) / new_cycles as f32;
                stone.rot_velocity = (new_rot - stone.rotation) / new_cycles as f32;
                stone.cycles = new_cycles;
            }
        } else {
            stone.x_offset += stone.x_velocity;
            stone.y_offset += stone.y_velocity;
            stone.rotation += stone.rot_velocity;
            stone.cycles -= 1;
        }
    }

    if model.recording && app.elapsed_frames() % 2 == 0 {
        model.cur_frame += 1;
        if model.cur_frame > 9999 {
            model.recording = false;
        } else {
            let filename = format!("{}/schotter{:>04}.png",
                model.frames_dir,
                model.cur_frame);
            app.main_window().save_screenshot(filename);
        }
    }
}

fn view(app: &App, model: &Model) {
    app.set_update_mode(UpdateMode::Continuous);
    let draw = app.draw();
    let gdraw = draw.scale(SIZE as f32)
                    .scale_y(-1.0)
                    .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    for stone in &model.gravel {
        let cdraw = gdraw.x_y(stone.x, stone.y);
        cdraw.rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x_offset, stone.y_offset)
            .rotate(stone.rotation)
            ;
    }
}

fn ui_view(_app: &App, _model: &Model) {}

fn key_pressed(app: &App, model: &mut Model, key: KeyCode) {
    match key {
        KeyCode::KeyS => {
            app.main_window()
                .save_screenshot(app.exe_name().unwrap() + ".png");
        }
        KeyCode::KeyR => {
            if model.recording {
                model.recording = false;
            } else {
                fs::create_dir(&model.frames_dir).unwrap_or_else(|error| {
                    if error.kind() != ErrorKind::AlreadyExists {
                        panic!{"Problem creating directory {:?}", model.frames_dir};
                    }
                });
                model.recording = true;
                model.cur_frame = 0;
            }
        }
        KeyCode::ArrowUp => {
            model.disp_adj += 0.1;
        }
        KeyCode::ArrowDown => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        KeyCode::ArrowRight => {
            model.rot_adj += 0.1;
        }
        KeyCode::ArrowLeft => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        KeyCode::Escape => {
            app.quit();
        }
        _other_key => {}
    }

}

fn update_ui(app: &App, model: &mut Model) {
    let ctx = app.egui_for_window(model.ui_window);
    egui::Window::new("Schotter Control Panel").collapsible(false).show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut model.disp_adj, 0.0..=5.0).text("Displacement"));
        ui.add(egui::Slider::new(&mut model.rot_adj, 0.0..=5.0).text("Rotation"));
        ui.add(egui::Slider::new(&mut model.motion, 0.0..=1.0).text("Motion"));
    });
}
