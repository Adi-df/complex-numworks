#![no_std]
#![no_main]
pub mod eadk;

use eadk::{
    display::{self, SCREEN_HEIGHT, SCREEN_WIDTH},
    key, keyboard, timing, Color, Point, Rect,
};

use core::fmt::Write;
use core::iter::FromIterator;

use heapless::String;
use libm::{expf, fabsf, floorf, log2f, truncf};

mod complex;
use complex::Complex;

mod function;
use function::{Evaluate, FastFunction, Function, MathInstruction, StringFunction, Validate};

#[export_name = "eadk_app_name"]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"ComplexNW\0";

#[export_name = "eadk_app_api_level"]
#[link_section = ".rodata.eadk_app_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[export_name = "eadk_app_icon"]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 3477] = *include_bytes!("../target/icon.nwi");

const CHARACTERS_BY_LINE: usize = 45;
const LINE_HEIGHT_IN_PIXEL: u16 = 14;

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
fn sigmoid_complex_to_color(z: Complex) -> Color {
    let value = (2. / (1. + expf(-z.modulus()))) - 1.;
    Color::from_hv(z.argument(), value)
}
fn checkerboard_complex_to_color(z: Complex) -> Color {
    Color::from_hv(
        z.argument(),
        if fabsf(floorf(z.real)) as u16 % 2 == fabsf(floorf(z.imag)) as u16 % 2 {
            0.5
        } else {
            1.
        },
    )
}

fn plot_rect(state: &State, rect: Rect) {
    let mut row: [Color; SCREEN_WIDTH as usize] = [Color::BLACK; SCREEN_WIDTH as usize];
    (rect.y..rect.height).for_each(|y| {
        let imag = (1. - y as f32 / SCREEN_HEIGHT as f32)
            * (state.area.to_imag - state.area.from_imag)
            + state.area.from_imag;

        (&mut row[0..rect.width as usize])
            .iter_mut()
            .enumerate()
            .for_each(move |(x, p)| {
                *p = (state.color_mode)(state.func.eval(Complex {
                    real: (x as f32 / SCREEN_WIDTH as f32)
                        * (state.area.to_real - state.area.from_real)
                        + state.area.from_real,
                    imag,
                }))
            });
        display::push_rect(
            Rect {
                x: rect.x,
                y,
                width: rect.width,
                height: 1,
            },
            &row,
        );
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

fn keyboard_number<const N: usize>(num: &mut String<N>) -> Option<f32> {
    let keyboard_state = keyboard::scan();

    if keyboard_state.key_down(key::ZERO) {
        num.push('0').unwrap_or(());
    } else if keyboard_state.key_down(key::ONE) {
        num.push('1').unwrap_or(());
    } else if keyboard_state.key_down(key::TWO) {
        num.push('2').unwrap_or(());
    } else if keyboard_state.key_down(key::THREE) {
        num.push('3').unwrap_or(());
    } else if keyboard_state.key_down(key::FOUR) {
        num.push('4').unwrap_or(());
    } else if keyboard_state.key_down(key::FIVE) {
        num.push('5').unwrap_or(());
    } else if keyboard_state.key_down(key::SIX) {
        num.push('6').unwrap_or(());
    } else if keyboard_state.key_down(key::SEVEN) {
        num.push('7').unwrap_or(());
    } else if keyboard_state.key_down(key::EIGHT) {
        num.push('8').unwrap_or(());
    } else if keyboard_state.key_down(key::NINE) {
        num.push('9').unwrap_or(());
    } else if keyboard_state.key_down(key::DOT) {
        num.push('.').unwrap_or(());
    } else if keyboard_state.key_down(key::MINUS) {
        match num.chars().nth(0) {
            Some('-') => *num = <String<N>>::from_iter(num.chars().skip(1)),
            None | Some(_) => *num = <String<N>>::from_iter(num.chars().rev().chain(['-']).rev()),
        }
    } else if keyboard_state.key_down(key::BACKSPACE) && num.len() > 0 {
        num.pop().unwrap();
    } else if keyboard_state.key_down(key::EXE) {
        return Some(num.as_str().parse::<f32>().unwrap());
    }
    None
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
                else if keyboard_state.key_down(key::ALPHA)
                    && keyboard_state.key_down(key::FIVE)
                    && state.color_mode != sigmoid_complex_to_color
                {
                    state.color_mode = sigmoid_complex_to_color;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::ALPHA)
                    && keyboard_state.key_down(key::FOUR)
                    && state.color_mode != checkerboard_complex_to_color
                {
                    state.color_mode = checkerboard_complex_to_color;

                    plot_func(&state);
                } else if keyboard_state.key_down(key::ALPHA)
                    && keyboard_state.key_down(key::SIX)
                    && state.color_mode != log2_complex_to_color
                {
                    state.color_mode = log2_complex_to_color;

                    plot_func(&state);
                }
                // Go to
                else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::SINE)
                {
                    let mut x: String<20> = String::new();
                    let mut y: String<20> = String::new();
                    let mut y_selected = false;

                    let x_margin = (state.area.to_real - state.area.from_real) / 2.;
                    let y_margin = (state.area.to_imag - state.area.from_imag) / 2.;

                    loop {
                        display::push_rect_uniform(
                            Rect {
                                x: 0,
                                y: 0,
                                width: SCREEN_WIDTH,
                                height: LINE_HEIGHT_IN_PIXEL * 2,
                            },
                            Color::WHITE,
                        );

                        let mut pos_str: String<50> = String::new();
                        write!(&mut pos_str, "x = {}\ny = {}\0", x, y).unwrap();
                        display::draw_string(
                            &pos_str,
                            Point::ZERO,
                            false,
                            Color::BLACK,
                            Color::WHITE,
                        );

                        if keyboard::scan().key_down(key::BACK) {
                            plot_rect(
                                &state,
                                Rect {
                                    x: 0,
                                    y: 0,
                                    width: SCREEN_WIDTH,
                                    height: LINE_HEIGHT_IN_PIXEL * 2,
                                },
                            );
                            break;
                        }

                        match y_selected {
                            false => {
                                if let Some(num) = keyboard_number(&mut x) {
                                    state.area.from_real = num - x_margin;
                                    state.area.to_real = num + x_margin;
                                    y_selected = true;

                                    while keyboard::scan().key_down(key::EXE) {}
                                }
                            }
                            true => {
                                if let Some(num) = keyboard_number(&mut y) {
                                    state.area.from_imag = num - y_margin;
                                    state.area.to_imag = num + y_margin;
                                    break;
                                }
                            }
                        }

                        timing::msleep(100);
                        display::wait_for_vblank();
                    }

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
                        height: LINE_HEIGHT_IN_PIXEL * 2,
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
                            height: LINE_HEIGHT_IN_PIXEL * 2,
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
                let mut max_line_count = 1;
                let previous_body = func_body.clone();

                loop {
                    let keyboard_state = keyboard::scan();

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

                    let mut line_count = 1;
                    let string: StringFunction = StringFunction::from(func_body.clone())
                        .split_inclusive(' ')
                        .into_iter()
                        .fold(StringFunction::new(), |mut str, el| {
                            if str.chars().count() % CHARACTERS_BY_LINE + el.chars().count()
                                > CHARACTERS_BY_LINE
                            {
                                line_count += 1;
                                str.push('\n').unwrap();
                            }
                            str.push_str(el).unwrap();
                            str
                        });
                    max_line_count = max_line_count.max(line_count);

                    display::push_rect_uniform(
                        Rect {
                            x: 0,
                            y: 0,
                            width: SCREEN_WIDTH,
                            height: max_line_count * LINE_HEIGHT_IN_PIXEL,
                        },
                        Color::WHITE,
                    );

                    display::draw_string(
                        string.as_str(),
                        Point::new(0, 0),
                        false,
                        Color::BLACK,
                        Color::WHITE,
                    );

                    if keyboard_state.key_down(key::SHIFT) && keyboard_state.key_down(key::EXP) {
                        func_body.push(MathInstruction::E).unwrap();
                    } else if keyboard_state.key_down(key::SHIFT)
                        && keyboard_state.key_down(key::SINE)
                    {
                        func_body.push(MathInstruction::Arcsin).unwrap();
                    } else if keyboard_state.key_down(key::SHIFT)
                        && keyboard_state.key_down(key::COSINE)
                    {
                        func_body.push(MathInstruction::Arccos).unwrap();
                    } else if keyboard_state.key_down(key::SHIFT)
                        && keyboard_state.key_down(key::TANGENT)
                    {
                        func_body.push(MathInstruction::Arctan).unwrap();
                    } else if keyboard_state.key_down(key::ALPHA)
                        && keyboard_state.key_down(key::MINUS)
                    {
                        func_body.push(MathInstruction::Conj).unwrap();
                    } else if keyboard_state.key_down(key::ALPHA)
                        && keyboard_state.key_down(key::XNT)
                    {
                        func_body.push(MathInstruction::ZConj).unwrap();
                    } else if keyboard_state.key_down(key::BACKSPACE) {
                        func_body.pop();
                    } else if keyboard_state.key_down(key::XNT) {
                        func_body.push(MathInstruction::Z).unwrap();
                    } else if keyboard_state.key_down(key::IMAGINARY) {
                        func_body.push(MathInstruction::Imag).unwrap();
                    } else if keyboard_state.key_down(key::PI) {
                        func_body.push(MathInstruction::Pi).unwrap();
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
                        func_body.push(MathInstruction::Ln).unwrap();
                    } else if keyboard_state.key_down(key::LOG) {
                        func_body.push(MathInstruction::Log).unwrap();
                    } else if keyboard_state.key_down(key::SINE) {
                        func_body.push(MathInstruction::Sin).unwrap();
                    } else if keyboard_state.key_down(key::COSINE) {
                        func_body.push(MathInstruction::Cos).unwrap();
                    } else if keyboard_state.key_down(key::TANGENT) {
                        func_body.push(MathInstruction::Tan).unwrap();
                    } else if keyboard_state.key_down(key::SQUARE) {
                        func_body.push(MathInstruction::Number(2.)).unwrap();
                        func_body.push(MathInstruction::Pow).unwrap();
                    } else if keyboard_state.key_down(key::SQRT) {
                        func_body.push(MathInstruction::Sqrt).unwrap();
                    } else if number_pressed {
                        let mut num: String<32> = String::new();
                        loop {
                            display::push_rect_uniform(
                                Rect {
                                    x: 0,
                                    y: 0,
                                    width: SCREEN_WIDTH,
                                    height: LINE_HEIGHT_IN_PIXEL,
                                },
                                Color::WHITE,
                            );

                            let mut num_str: String<33> = String::new();
                            write!(&mut num_str, "{}\0", num).unwrap();
                            display::draw_string(
                                &num_str,
                                Point::ZERO,
                                false,
                                Color::BLACK,
                                Color::WHITE,
                            );

                            if let Some(num) = keyboard_number(&mut num) {
                                func_body.push(MathInstruction::Number(num)).unwrap();
                                break;
                            }

                            timing::msleep(100);
                            display::wait_for_vblank();
                        }
                    } else if keyboard_state.key_down(key::BACK) {
                        func_body = previous_body;
                        state.mode = StateMode::Default;
                        plot_rect(
                            &state,
                            Rect {
                                x: 0,
                                y: 0,
                                width: SCREEN_WIDTH,
                                height: max_line_count * LINE_HEIGHT_IN_PIXEL,
                            },
                        );
                        break;
                    } else if keyboard_state.key_down(key::OK) {
                        match func_body.validate() {
                            Ok(()) => {
                                state.func = FastFunction::from(func_body.clone());
                                state.mode = StateMode::Default;

                                plot_func(&state);
                                break;
                            }
                            Err(_) => {
                                display::draw_string(
                                    string.as_str(),
                                    Point::new(0, 0),
                                    false,
                                    Color::RED,
                                    Color::WHITE,
                                );
                                timing::msleep(400);
                            }
                        }
                    }

                    timing::msleep(100);
                    display::wait_for_vblank();
                }
            }
        }
    }
}
