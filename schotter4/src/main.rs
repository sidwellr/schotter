use nannou::prelude::*;
use nannou::ui::prelude::*;
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

widget_ids! {
    struct Ids {
        title,
        disp_label,
        disp_slider,
        rot_label,
        rot_slider,
        motion_label,
        motion_slider,
    }
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
    ui: Ui,
    ids: Ids,
    main_window: WindowId,
    frames_dir: String,
    cur_frame: u32,
    recording: bool,
    disp_adj: f32,
    rot_adj: f32,
    motion: f32,
    gravel: Vec<Stone>
}

fn model(app: &App) -> Model {
    let main_window = app.new_window()
                .title("Schotter4")
                .size(WIDTH, HEIGHT)
                .view(view)
                .key_pressed(key_pressed)
                .build()
                .unwrap();

    let ui_window = app.new_window()
                .title("Schotter4 controls")
                .size(300, 200)
                .view(ui_view)
                .event(ui_event)
                .key_pressed(key_pressed)
                .build()
                .unwrap();

    let mut ui = app.new_ui().window(ui_window).build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    ui.clear_with(nannou::ui::prelude::color::DARK_CHARCOAL);
    let mut theme = ui.theme_mut();
    theme.label_color = nannou::ui::prelude::color::WHITE;
    theme.shape_color = nannou::ui::prelude::color::CHARCOAL;

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
            gravel.push(stone)
        }
    }

    let mut the_model = Model {
        ui,
        ids,
        main_window,
        frames_dir,
        recording,
        cur_frame,
        disp_adj,
        rot_adj,
        motion,
        gravel,
    };

    // Send a fake ui_event to draw widgets
    ui_event(&app, &mut the_model, WindowEvent::Focused);

    the_model
}

fn update(app: &App, model: &mut Model, _update: Update) {
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
            match app.window(model.main_window) {
                Some(window) => {
                    window.capture_frame(filename);
                }
                None => {}
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let gdraw = draw.scale(SIZE as f32)
                    .scale_y(-1.0)
                    .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5);

    draw.background().color(SNOW);

    for stone in &model.gravel {
        gdraw.rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x + stone.x_offset, stone.y + stone.y_offset)
            .rotate(stone.rotation)
            ;
    }

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            match app.window(model.main_window) {
                Some(window) => {
                    window.capture_frame(app.exe_name().unwrap() + ".png");
                }
                None => {}
            }
        }
        Key::R => {
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

fn ui_event(_app: &App, model: &mut Model, _event: WindowEvent) {
    let ui = &mut model.ui.set_widgets();

    // Control panel title
    widget::Text::new("Schotter Control Panel")
        .top_left_with_margin(10.0)
        .w_h(300.0, 40.0)
        .font_size(24)
        .set(model.ids.title, ui);

    // Displacement label
    widget::Text::new("Displacement")
        .down_from(model.ids.title, 15.0)
        .w_h(125.0, 30.0)
        .set(model.ids.disp_label, ui);

    // Displacement slider
    for value in widget::Slider::new(model.disp_adj, 0.0, 5.0)
        .right_from(model.ids.disp_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.disp_adj.to_string())
        .set(model.ids.disp_slider, ui)
    {
        model.disp_adj = value;
    }

    // Rotation label
    widget::Text::new("Rotation")
        .down_from(model.ids.disp_label, 10.00)
        .w_h(125.0, 30.0)
        .set(model.ids.rot_label, ui);

    // Rotation slider
    for value in widget::Slider::new(model.rot_adj, 0.0, 5.0)
        .right_from(model.ids.rot_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.rot_adj.to_string())
        .set(model.ids.rot_slider, ui)
    {
        model.rot_adj = value;
    }

    // Motion label
    widget::Text::new("Motion")
        .down_from(model.ids.rot_label, 10.00)
        .w_h(125.0, 30.0)
        .set(model.ids.motion_label, ui);

    // Motion slider
    for value in widget::Slider::new(model.motion, 0.0, 1.0)
        .right_from(model.ids.motion_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.motion.to_string())
        .set(model.ids.motion_slider, ui)
    {
        model.motion = value;
    }
}

fn ui_view(app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame_if_changed(app, &frame).unwrap();
}
