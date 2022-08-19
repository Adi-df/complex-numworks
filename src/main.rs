#![no_std]
#![no_main]
pub mod eadk;

use eadk::{
    display::{SCREEN_HEIGHT, SCREEN_WIDTH},
    key, keyboard, Color,
};

use libm::{expf, fabsf, floorf, log2f, truncf};

mod complex;
use complex::{Complex, ComplexRect};

mod function;
use function::{FastFunction, Function, MathInstruction};

mod plot;
mod utils;
use plot::plot_func;

mod editor;
mod goto;
mod values;

#[export_name = "eadk_app_name"]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 10] = *b"ComplexNW\0";

#[export_name = "eadk_app_api_level"]
#[link_section = ".rodata.eadk_app_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[export_name = "eadk_app_icon"]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 3477] = *include_bytes!("../target/icon.nwi");

pub const CHARACTERS_BY_LINE: usize = 45;
pub const LINE_HEIGHT_IN_PIXEL: u16 = 14;

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

pub struct State {
    func: FastFunction,
    func_body: Function,
    area: ComplexRect,
    color_mode: fn(Complex) -> Color,
}

#[no_mangle]
fn _eadk_main() {
    let mut state = {
        let func_body = Function::from_slice(&[MathInstruction::Z]);
        State {
            func: FastFunction::from(func_body.clone()),
            func_body,
            area: ComplexRect {
                from_real: -10.,
                to_real: 10.,
                from_imag: -10.,
                to_imag: 10.,
            },
            color_mode: sigmoid_complex_to_color,
        }
    };

    plot_func(&state);

    loop {
        let keyboard_state = keyboard::scan();

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
        else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::COMMA) {
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
        else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::SINE) {
            goto::goto(&mut state);
        }
        // Explore values
        else if keyboard_state.key_down(key::VAR) {
            values::values(&mut state);
        }
        //Editor
        else if keyboard_state.key_down(key::TOOLBOX) {
            editor::editor(&mut state);
        }
    }
}
