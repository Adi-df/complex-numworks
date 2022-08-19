use crate::eadk::display::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::complex::{Complex, ComplexRect};

pub fn map_to_complex(area: &ComplexRect, pos: (u16, u16)) -> Complex {
    Complex {
        real: (pos.0 as f32 / SCREEN_WIDTH as f32) * (area.to_real - area.from_real)
            + area.from_real,
        imag: (1. - pos.1 as f32 / SCREEN_HEIGHT as f32) * (area.to_imag - area.from_imag)
            + area.from_imag,
    }
}
