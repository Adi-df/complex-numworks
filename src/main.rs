#![no_std]
#![no_main]
pub mod eadk;

use eadk::{
    display::{self, SCREEN_HEIGHT, SCREEN_WIDTH},
    key, keyboard, timing, Color, Point, Rect,
};

use core::fmt::Write;

use heapless::String;
use libm::{expf, fabsf, log10f, log2f, truncf};

mod complex;
use complex::Complex;

mod function;
use function::{Evaluate, FastFunction, Function, MathInstruction, StringFunction};

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
        real: (pos.0 as f32 / SCREEN_WIDTH as f32) * (area.to_real - area.from_real)
            + area.from_real,
        imag: (1. - pos.1 as f32 / SCREEN_HEIGHT as f32) * (area.to_imag - area.from_imag)
            + area.from_imag,
    }
}

fn log2_complex_to_color(z: Complex) -> Color {
    let value = fabsf(log2f(z.modulus()));
    Color::from_hv(z.argument(), value - truncf(value))
}
// fn log10_complex_to_color(z: Complex) -> Color {
//     let value = fabs(log10(z.modulus()));
//     Color::from_hv(z.argument(), value - trunc(value))
// }
fn sigmoid_complex_to_color(z: Complex) -> Color {
    let value = (2. / (1. + expf(-z.modulus()))) - 1.;
    Color::from_hv(z.argument(), value)
}

fn plot_rect(state: &State, rect: Rect) {
    (rect.y..rect.height).for_each(|y| {
        let imag = (1. - y as f32 / SCREEN_HEIGHT as f32)
            * (state.area.to_imag - state.area.from_imag)
            + state.area.from_imag;
        (rect.x..rect.width)
            .map(move |x| {
                (x, {
                    state.func.eval(Complex {
                        real: (x as f32 / SCREEN_WIDTH as f32)
                            * (state.area.to_real - state.area.from_real)
                            + state.area.from_real,
                        imag,
                    })
                })
            })
            .for_each(|(x, z)| {
                display::push_rect(
                    Rect {
                        x: x as u16,
                        y: y as u16,
                        width: 1,
                        height: 1,
                    },
                    &[(state.color_mode)(z)],
                );
            });
    });
}

fn plot_func(state: &State) {
    plot_rect(
        state,
        Rect {
            x: 0,
            y: 0,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        },
    );
}

#[derive(Clone)]
struct ComplexRect {
    from_real: f32,
    to_real: f32,
    from_imag: f32,
    to_imag: f32,
}

struct State {
    func: FastFunction,
    area: ComplexRect,
    color_mode: fn(Complex) -> Color,
    mode: StateMode,
}

enum StateMode {
    Default,
    ValueExplorer { x: u16, y: u16 },
    FunctionEditor,
}

#[no_mangle]
fn _eadk_main() {
    let mut func_body = Function::from_slice(&[MathInstruction::Z]);

    let color_modes = [
        log2_complex_to_color,
        // log10_complex_to_color,
        sigmoid_complex_to_color,
    ];

    let mut state = State {
        func: FastFunction::from(func_body.clone()),
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
                        * (SCREEN_HEIGHT as f32 / SCREEN_WIDTH as f32)
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
                    state.mode = StateMode::FunctionEditor;
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

                    plot_rect(
                        &state,
                        Rect {
                            x: 0,
                            y: 0,
                            width: SCREEN_WIDTH,
                            height: 30,
                        },
                    );

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
            StateMode::FunctionEditor => {
                let number_pressed = keyboard_state.key_down(key::ZERO)
                    || keyboard_state.key_down(key::ONE)
                    || keyboard_state.key_down(key::TWO)
                    || keyboard_state.key_down(key::THREE)
                    || keyboard_state.key_down(key::FOUR)
                    || keyboard_state.key_down(key::FIVE)
                    || keyboard_state.key_down(key::SIX)
                    || keyboard_state.key_down(key::SEVEN)
                    || keyboard_state.key_down(key::EIGHT)
                    || keyboard_state.key_down(key::NINE);

                display::push_rect_uniform(
                    Rect {
                        x: 0,
                        y: 0,
                        width: SCREEN_WIDTH,
                        height: 30,
                    },
                    Color::WHITE,
                );
                display::draw_string(
                    StringFunction::from(func_body.clone()).as_str(),
                    Point::new(0, 0),
                    false,
                    Color::BLACK,
                    Color::WHITE,
                );

                if keyboard_state.key_down(key::BACKSPACE) {
                    func_body.pop();
                } else if keyboard_state.key_down(key::XNT) {
                    func_body.push(MathInstruction::Z).unwrap();
                } else if keyboard_state.key_down(key::IMAGINARY) {
                    func_body.push(MathInstruction::Imag).unwrap();
                } else if keyboard_state.key_down(key::PI) {
                    func_body.push(MathInstruction::Pi).unwrap();
                } else if keyboard_state.key_down(key::SHIFT) && keyboard_state.key_down(key::EXP) {
                    func_body.push(MathInstruction::E).unwrap();
                } else if keyboard_state.key_down(key::PLUS) {
                    func_body.push(MathInstruction::Add).unwrap();
                } else if keyboard_state.key_down(key::MINUS) {
                    func_body.push(MathInstruction::Sub).unwrap();
                } else if keyboard_state.key_down(key::MULTIPLICATION) {
                    func_body.push(MathInstruction::Mul).unwrap();
                } else if keyboard_state.key_down(key::DIVISION) {
                    func_body.push(MathInstruction::Div).unwrap();
                } else if keyboard_state.key_down(key::POWER) {
                    func_body.push(MathInstruction::Pow).unwrap();
                } else if keyboard_state.key_down(key::EXP) {
                    func_body.push(MathInstruction::Exp).unwrap();
                } else if keyboard_state.key_down(key::LN) {
                    func_body.push(MathInstruction::Log).unwrap();
                } else if keyboard_state.key_down(key::SINE) {
                    func_body.push(MathInstruction::Sin).unwrap();
                } else if keyboard_state.key_down(key::COSINE) {
                    func_body.push(MathInstruction::Cos).unwrap();
                } else if keyboard_state.key_down(key::SQUARE) {
                    func_body.push(MathInstruction::Number(2.)).unwrap();
                    func_body.push(MathInstruction::Pow).unwrap();
                } else if number_pressed {
                    let mut num: String<32> = String::new();
                    loop {
                        display::push_rect_uniform(
                            Rect {
                                x: 0,
                                y: 0,
                                width: SCREEN_WIDTH,
                                height: 30,
                            },
                            Color::WHITE,
                        );
                        display::draw_string(&num, Point::ZERO, false, Color::BLACK, Color::WHITE);

                        let keyboard_state = keyboard::scan();

                        num.pop();
                        if keyboard_state.key_down(key::ZERO) {
                            num.push('0').unwrap();
                        } else if keyboard_state.key_down(key::ONE) {
                            num.push('1').unwrap();
                        } else if keyboard_state.key_down(key::TWO) {
                            num.push('2').unwrap();
                        } else if keyboard_state.key_down(key::THREE) {
                            num.push('3').unwrap();
                        } else if keyboard_state.key_down(key::FOUR) {
                            num.push('4').unwrap();
                        } else if keyboard_state.key_down(key::FIVE) {
                            num.push('5').unwrap();
                        } else if keyboard_state.key_down(key::SIX) {
                            num.push('6').unwrap();
                        } else if keyboard_state.key_down(key::SEVEN) {
                            num.push('7').unwrap();
                        } else if keyboard_state.key_down(key::EIGHT) {
                            num.push('8').unwrap();
                        } else if keyboard_state.key_down(key::NINE) {
                            num.push('9').unwrap();
                        } else if keyboard_state.key_down(key::DOT) {
                            num.push('.').unwrap();
                        } else if keyboard_state.key_down(key::BACKSPACE) {
                            num.pop();
                        } else if keyboard_state.key_down(key::EXE) {
                            func_body
                                .push(MathInstruction::Number(num.parse().unwrap()))
                                .unwrap();
                            break;
                        }
                        num.push('\0').unwrap();

                        timing::msleep(100);
                        display::wait_for_vblank();
                    }
                } else if keyboard_state.key_down(key::BACK) {
                    state.mode = StateMode::Default;
                    plot_rect(
                        &state,
                        Rect {
                            x: 0,
                            y: 0,
                            width: SCREEN_WIDTH,
                            height: 30,
                        },
                    );
                } else if keyboard_state.key_down(key::OK) {
                    state.func = FastFunction::from(func_body.clone());
                    state.mode = StateMode::Default;

                    plot_func(&state);
                }

                timing::msleep(100);
                display::wait_for_vblank();
            }
        }
    }
}
