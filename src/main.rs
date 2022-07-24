#![no_std]
#![no_main]
pub mod eadk;

use eadk::{
    display::{self, SCREEN_HEIGHT, SCREEN_WIDTH},
    key, keyboard, timing, Color, Point, Rect,
};

use core::fmt::Write;

use heapless::String;
use libm::{exp, fabs, log10, log2};

mod complex;
use complex::Complex;

mod function;
use function::{Evaluate, Function, StringFunction};

#[export_name = "eadk_app_name"]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"HelloRust\0";

#[export_name = "eadk_app_api_level"]
#[link_section = ".rodata.eadk_app_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[export_name = "eadk_app_icon"]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

fn map_to_complex(area: &ComplexRect, pos: (u16, u16)) -> Complex {
    Complex {
        real: (pos.0 as f64 / SCREEN_WIDTH as f64) * (area.to_real - area.from_real)
            + area.from_real,
        imag: (1. - pos.1 as f64 / SCREEN_HEIGHT as f64) * (area.to_imag - area.from_imag)
            + area.from_imag,
    }
}

fn log2_complex_to_color(z: Complex) -> Color {
    let (hue, saturation, value) = (z.argument(), 1., fabs(log2(z.modulus())) % 1.);
    Color::from_hsv(hue, saturation, value)
}
fn log10_complex_to_color(z: Complex) -> Color {
    let (hue, saturation, value) = (z.argument(), 1., fabs(log10(z.modulus())) % 1.);
    Color::from_hsv(hue, saturation, value)
}
fn sigmoid_complex_to_color(z: Complex) -> Color {
    let (hue, saturation, value) = (z.argument(), 1., (1. / (1. + exp(-z.modulus()))) * 2. - 1.);
    Color::from_hsv(hue, saturation, value)
}

fn plot_func(state: &State) {
    (0..eadk::display::SCREEN_WIDTH).for_each(|x| {
        let real = (x as f64 / SCREEN_WIDTH as f64) * (state.area.to_real - state.area.from_real)
            + state.area.from_real;
        (0..eadk::display::SCREEN_HEIGHT)
            .map(move |y| Complex {
                real,
                imag: (1. - y as f64 / SCREEN_HEIGHT as f64)
                    * (state.area.to_imag - state.area.from_imag)
                    + state.area.from_imag,
            })
            .map(|z| state.func.eval(z))
            .enumerate()
            .for_each(|(y, z)| {
                display::push_rect_uniform(
                    Rect {
                        x,
                        y: y as u16,
                        width: 1,
                        height: 1,
                    },
                    (state.color_mode)(z),
                );
            });
    });
}

#[derive(Clone)]
struct ComplexRect {
    from_real: f64,
    to_real: f64,
    from_imag: f64,
    to_imag: f64,
}

struct State<'a> {
    func: &'a dyn Evaluate,
    area: ComplexRect,
    color_mode: fn(Complex) -> Color,
    mode: StateMode,
}

enum StateMode {
    Default,
    ValueExplorer { x: u16, y: u16 },
    FunctionEditor { instructions: Function },
}

#[no_mangle]
fn _eadk_main() {
    let func = |z: Complex| z;

    let color_modes = [
        log2_complex_to_color,
        log10_complex_to_color,
        sigmoid_complex_to_color,
    ];

    let mut state = State {
        func: &func,
        area: ComplexRect {
            from_real: -10.,
            to_real: 10.,
            from_imag: -10.,
            to_imag: 10.,
        },
        color_mode: sigmoid_complex_to_color,
        mode: StateMode::Default,
    };

    plot_func(&state);

    loop {
        let keyboard_state = keyboard::scan();

        match state.mode {
            StateMode::Default => {
                if keyboard_state.key_down(key::HOME) {
                    break;
                } else if keyboard_state.key_down(key::PLUS) {
                    state.area.from_real /= 2.;
                    state.area.to_real /= 2.;
                    state.area.from_imag /= 2.;
                    state.area.to_imag /= 2.;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::MINUS) {
                    state.area.from_real *= 2.;
                    state.area.to_real *= 2.;
                    state.area.from_imag *= 2.;
                    state.area.to_imag *= 2.;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::LEFT) {
                    let shift = (state.area.to_real - state.area.from_real) / 5.;
                    state.area.from_real -= shift;
                    state.area.to_real -= shift;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::RIGHT) {
                    let shift = (state.area.to_real - state.area.from_real) / 5.;
                    state.area.from_real += shift;
                    state.area.to_real += shift;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::DOWN) {
                    let shift = (state.area.to_imag - state.area.from_imag) / 5.;
                    state.area.from_imag -= shift;
                    state.area.to_imag -= shift;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::UP) {
                    let shift = (state.area.to_imag - state.area.from_imag) / 5.;
                    state.area.from_imag += shift;
                    state.area.to_imag += shift;

                    plot_func(&state);
                }
                // Equal axes
                else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::COMMA)
                {
                    let shift = (state.area.to_real - state.area.from_real)
                        * (SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64)
                        / 2.;
                    let mean = (state.area.to_imag + state.area.from_imag) / 2.;

                    state.area.from_imag = mean - shift;
                    state.area.to_imag = mean + shift;

                    plot_func(&state);
                }
                // Style
                else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::FIVE)
                {
                    state.color_mode = color_modes[(color_modes
                        .iter()
                        .position(|mode| *mode == state.color_mode)
                        .unwrap_or(0)
                        + 1)
                        % color_modes.len()];

                    plot_func(&state);
                }
                // Modes
                else if keyboard_state.key_down(key::VAR) {
                    state.mode = StateMode::ValueExplorer {
                        x: SCREEN_WIDTH / 2,
                        y: SCREEN_HEIGHT / 2,
                    }
                } else if keyboard_state.key_down(key::TOOLBOX) {
                    state.mode = StateMode::FunctionEditor {
                        instructions: Function::new(),
                    }
                }
            }
            StateMode::ValueExplorer { x, y } => {
                let (mut x, mut y) = (x, y);
                let z = map_to_complex(&state.area, (x, y));
                let fz = state.func.eval(z);

                let mut s: String<100> = String::new();
                write!(&mut s, "z = {z}\nf(z) = {fz}\0").unwrap();
                display::push_rect_uniform(
                    Rect {
                        x: 0,
                        y: 0,
                        width: SCREEN_WIDTH,
                        height: 30,
                    },
                    Color::WHITE,
                );
                display::draw_string(&s, Point::new(0, 0), false, Color::BLACK, Color::WHITE);
                display::push_rect_uniform(
                    Rect {
                        x,
                        y,
                        width: 1,
                        height: 1,
                    },
                    (state.color_mode)(fz),
                );

                if keyboard_state.key_down(key::RIGHT) {
                    x += 1;
                } else if keyboard_state.key_down(key::LEFT) {
                    x -= 1;
                }

                if keyboard_state.key_down(key::UP) {
                    y -= 1;
                } else if keyboard_state.key_down(key::DOWN) {
                    y += 1;
                } else if keyboard_state.key_down(key::BACK) {
                    state.mode = StateMode::Default;
                    timing::msleep(200);

                    (0..eadk::display::SCREEN_WIDTH).for_each(|x| {
                        let real = (x as f64 / SCREEN_WIDTH as f64)
                            * (state.area.to_real - state.area.from_real)
                            + state.area.from_real;
                        (0..30)
                            .map(|y| Complex {
                                real,
                                imag: (1. - y as f64 / SCREEN_HEIGHT as f64)
                                    * (state.area.to_imag - state.area.from_imag)
                                    + state.area.from_imag,
                            })
                            .map(|z| state.func.eval(z))
                            .enumerate()
                            .for_each(|(y, z)| {
                                display::push_rect_uniform(
                                    Rect {
                                        x,
                                        y: y as u16,
                                        width: 1,
                                        height: 1,
                                    },
                                    (state.color_mode)(z),
                                );
                            });
                    });

                    continue;
                }

                display::push_rect_uniform(
                    Rect {
                        x,
                        y,
                        width: 1,
                        height: 1,
                    },
                    Color::WHITE,
                );

                state.mode = StateMode::ValueExplorer { x, y };

                display::wait_for_vblank();
                timing::msleep(50);
            }
            StateMode::FunctionEditor { ref instructions } => {
                display::draw_string(
                    <&Function as Into<StringFunction>>::into(instructions).as_str(),
                    Point::new(0, 10),
                    false,
                    Color::BLACK,
                    Color::WHITE,
                );
            }
        }
    }
}
