use core::fmt::Write;

use heapless::String;

use crate::eadk::{
    display::{self, SCREEN_HEIGHT, SCREEN_WIDTH},
    key, keyboard, timing, Color, Point, Rect,
};

use crate::{map_to_complex, plot_rect, State, LINE_HEIGHT_IN_PIXEL};

use crate::function::Evaluate;

pub fn values(state: &mut State) {
    let (mut x, mut y) = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

    loop {
        let keyboard_state = keyboard::scan();

        let z = map_to_complex(&state.area, (x, y));
        let fz = state.func.eval(z);

        let mut s: String<256> = String::new();
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

        display::push_rect_uniform(
            Rect {
                x,
                y,
                width: 1,
                height: 1,
            },
            Color::WHITE,
        );

        display::wait_for_vblank();
        timing::msleep(50);
    }
}
