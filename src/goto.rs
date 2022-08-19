use core::fmt::Write;

use heapless::String;

use crate::eadk::{
    display::{self, SCREEN_WIDTH},
    key, keyboard, timing, Color, Point, Rect,
};

use crate::plot::{plot_func, plot_rect};
use crate::utils::keyboard_number;

use crate::{State, LINE_HEIGHT_IN_PIXEL};

pub fn goto(state: &mut State) {
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
        display::draw_string(&pos_str, Point::ZERO, false, Color::BLACK, Color::WHITE);

        if keyboard::scan().key_down(key::BACK) {
            plot_rect(
                state,
                Rect {
                    x: 0,
                    y: 0,
                    width: SCREEN_WIDTH,
                    height: LINE_HEIGHT_IN_PIXEL * 2,
                },
            );
            break;
        }

        if !y_selected {
            if let Some(num) = keyboard_number(&mut x) {
                state.area.from_real = num - x_margin;
                state.area.to_real = num + x_margin;
                y_selected = true;
            }
        } else if let Some(num) = keyboard_number(&mut y) {
            state.area.from_imag = num - y_margin;
            state.area.to_imag = num + y_margin;
            break;
        }

        timing::msleep(100);
        display::wait_for_vblank();
    }

    plot_func(state);
}
