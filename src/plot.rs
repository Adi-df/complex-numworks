use libm::{fabsf, floorf, log2f, tanhf, truncf};

use crate::eadk::display::{self, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::eadk::{Color, Rect};

use crate::complex::Complex;

use crate::function::Evaluate;

use crate::State;

pub fn plot_rect(state: &State, rect: Rect) {
    let color_mapper = state.color_mode.mapper();

    let mut row: [Color; SCREEN_WIDTH as usize] = [Color::BLACK; SCREEN_WIDTH as usize];
    (rect.y..rect.height).for_each(|y| {
        let imag = (1. - y as f32 / SCREEN_HEIGHT as f32)
            * (state.area.to_imag - state.area.from_imag)
            + state.area.from_imag;

        (&mut row[0..rect.width as usize])
            .iter_mut()
            .enumerate()
            .for_each(move |(x, p)| {
                *p = color_mapper(state.func.eval(Complex {
                    real: (x as f32 / SCREEN_WIDTH as f32)
                        * (state.area.to_real - state.area.from_real)
                        + state.area.from_real,
                    imag,
                }));
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

pub fn plot_func(state: &State) {
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

pub mod complex_to_color {
    use super::{fabsf, floorf, log2f, tanhf, truncf, Color, Complex};

    #[derive(PartialEq, Eq, Clone, Copy)]
    pub enum ColorMapper {
        Sigmoid,
        Log2,
        Checkerboard,
    }
    impl ColorMapper {
        pub fn mapper(self) -> fn(Complex) -> Color {
            match self {
                ColorMapper::Sigmoid => sigmoid,
                ColorMapper::Log2 => log2,
                ColorMapper::Checkerboard => checkerboard,
            }
        }
    }

    pub fn log2(z: Complex) -> Color {
        let value = fabsf(log2f(z.modulus()));
        Color::from_hv(z.argument(), value - truncf(value))
    }
    pub fn sigmoid(z: Complex) -> Color {
        let value = tanhf(z.modulus());
        Color::from_hv(z.argument(), value)
    }
    pub fn checkerboard(z: Complex) -> Color {
        Color::from_hv(
            z.argument(),
            if fabsf(floorf(z.real)) as u16 % 2 == fabsf(floorf(z.imag)) as u16 % 2 {
                0.5
            } else {
                1.
            },
        )
    }
}
