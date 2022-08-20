use core::fmt::Write;

use heapless::String;

use crate::eadk::display::{self, SCREEN_WIDTH};
use crate::eadk::timing;
use crate::eadk::{key, keyboard};
use crate::eadk::{Color, Point, Rect};

use crate::function::{FastFunction, MathInstruction, StringFunction, SyntaxError, Validate};

use crate::plot::{plot_func, plot_rect};
use crate::utils::{keyboard_number, CHARACTER_WIDTH};
use crate::utils::{CHARACTERS_BY_LINE, CHARACTER_HEIGHT};

use crate::State;

pub fn editor(state: &mut State) {
    let mut max_line_count = 1;
    let previous_body = state.func_body.clone();

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
        let string: StringFunction = StringFunction::from(state.func_body.clone())
            .split_inclusive(' ')
            .into_iter()
            .fold(StringFunction::new(), |mut str, el| {
                if str.chars().count() % CHARACTERS_BY_LINE + el.chars().count()
                    >= CHARACTERS_BY_LINE
                {
                    line_count += 1;
                    str.push('\n').unwrap();
                }
                str.push_str(el).unwrap();
                str
            });

        if keyboard_state.any_down() || line_count > max_line_count {
            display::push_rect_uniform(
                Rect {
                    x: 0,
                    y: 0,
                    width: SCREEN_WIDTH,
                    height: max_line_count.max(line_count) * CHARACTER_HEIGHT,
                },
                Color::WHITE,
            );
        }

        max_line_count = max_line_count.max(line_count);

        display::draw_string(
            string.as_str(),
            Point::ZERO,
            false,
            Color::BLACK,
            Color::WHITE,
        );

        if keyboard_state.key_down(key::SHIFT) && keyboard_state.key_down(key::EXP) {
            state.func_body.push(MathInstruction::E).unwrap();
        } else if keyboard_state.key_down(key::SHIFT) && keyboard_state.key_down(key::SINE) {
            state.func_body.push(MathInstruction::Arcsin).unwrap();
        } else if keyboard_state.key_down(key::SHIFT) && keyboard_state.key_down(key::COSINE) {
            state.func_body.push(MathInstruction::Arccos).unwrap();
        } else if keyboard_state.key_down(key::SHIFT) && keyboard_state.key_down(key::TANGENT) {
            state.func_body.push(MathInstruction::Arctan).unwrap();
        } else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::MINUS) {
            state.func_body.push(MathInstruction::Conj).unwrap();
        } else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::XNT) {
            state.func_body.push(MathInstruction::ConjZ).unwrap();
        } else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::FOUR) {
            state.func_body.push(MathInstruction::Re).unwrap();
        } else if keyboard_state.key_down(key::ALPHA) && keyboard_state.key_down(key::TANGENT) {
            state.func_body.push(MathInstruction::Im).unwrap();
        } else if keyboard_state.key_down(key::BACKSPACE) {
            state.func_body.pop();
        } else if keyboard_state.key_down(key::XNT) {
            state.func_body.push(MathInstruction::Z).unwrap();
        } else if keyboard_state.key_down(key::IMAGINARY) {
            state.func_body.push(MathInstruction::Imag).unwrap();
        } else if keyboard_state.key_down(key::PI) {
            state.func_body.push(MathInstruction::Pi).unwrap();
        } else if keyboard_state.key_down(key::PLUS) {
            state.func_body.push(MathInstruction::Add).unwrap();
        } else if keyboard_state.key_down(key::MINUS) {
            state.func_body.push(MathInstruction::Sub).unwrap();
        } else if keyboard_state.key_down(key::MULTIPLICATION) {
            state.func_body.push(MathInstruction::Mul).unwrap();
        } else if keyboard_state.key_down(key::DIVISION) {
            state.func_body.push(MathInstruction::Div).unwrap();
        } else if keyboard_state.key_down(key::POWER) {
            state.func_body.push(MathInstruction::Pow).unwrap();
        } else if keyboard_state.key_down(key::EXP) {
            state.func_body.push(MathInstruction::Exp).unwrap();
        } else if keyboard_state.key_down(key::LN) {
            state.func_body.push(MathInstruction::Ln).unwrap();
        } else if keyboard_state.key_down(key::LOG) {
            state.func_body.push(MathInstruction::Log).unwrap();
        } else if keyboard_state.key_down(key::SINE) {
            state.func_body.push(MathInstruction::Sin).unwrap();
        } else if keyboard_state.key_down(key::COSINE) {
            state.func_body.push(MathInstruction::Cos).unwrap();
        } else if keyboard_state.key_down(key::TANGENT) {
            state.func_body.push(MathInstruction::Tan).unwrap();
        } else if keyboard_state.key_down(key::SQUARE) {
            state.func_body.push(MathInstruction::Number(2.)).unwrap();
            state.func_body.push(MathInstruction::Pow).unwrap();
        } else if keyboard_state.key_down(key::SQRT) {
            state.func_body.push(MathInstruction::Sqrt).unwrap();
        } else if number_pressed {
            let mut num: String<32> = String::new();
            loop {
                display::push_rect_uniform(
                    Rect {
                        x: 0,
                        y: 0,
                        width: SCREEN_WIDTH,
                        height: CHARACTER_HEIGHT,
                    },
                    Color::BLACK,
                );

                let mut num_str: String<33> = String::new();
                write!(&mut num_str, "{}\0", num).unwrap();
                display::draw_string(&num_str, Point::ZERO, false, Color::WHITE, Color::BLACK);

                if let Some(num) = keyboard_number(&mut num) {
                    state.func_body.push(MathInstruction::Number(num)).unwrap();
                    break;
                }

                timing::msleep(100);
                display::wait_for_vblank();
            }
        } else if keyboard_state.key_down(key::BACK) {
            state.func_body = previous_body;
            plot_rect(
                state,
                Rect {
                    x: 0,
                    y: 0,
                    width: SCREEN_WIDTH,
                    height: max_line_count * CHARACTER_HEIGHT,
                },
            );
            break;
        } else if keyboard_state.key_down(key::OK) {
            if let Err(SyntaxError { op_index: index }) = state.func_body.validate() {
                if index == usize::MAX {
                    display::draw_string(
                        string.as_str(),
                        Point::new(0, 0),
                        false,
                        Color::RED,
                        Color::WHITE,
                    );
                    timing::msleep(400);
                } else {
                    let (x, y) = string
                        .split_inclusive(' ')
                        .into_iter()
                        .scan((0, 0), |(x, y), el| {
                            el.chars().for_each(|c| {
                                if c == '\n' {
                                    *x = 0;
                                    *y += 1;
                                } else {
                                    *x += 1;
                                }
                            });
                            Some((*x, *y))
                        })
                        .nth(index)
                        .unwrap();

                    let mut syntax_error_str: String<16> = String::new();
                    write!(&mut syntax_error_str, "{}\0", state.func_body[index]).unwrap();

                    display::draw_string(
                        string.as_str(),
                        Point::new(0, 0),
                        false,
                        Color::BLACK,
                        Color::WHITE,
                    );
                    display::draw_string(
                        syntax_error_str.as_str(),
                        Point::new(x as u16 * CHARACTER_WIDTH, y as u16 * CHARACTER_HEIGHT),
                        false,
                        Color::RED,
                        Color::WHITE,
                    );
                }
            } else {
                state.func = FastFunction::from(state.func_body.clone());

                plot_func(state);
                break;
            }
        }

        timing::msleep(100);
        display::wait_for_vblank();
    }
}
