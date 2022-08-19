use heapless::String;

use crate::eadk::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::eadk::{key, keyboard};

use crate::complex::{Complex, ComplexRect};

pub fn map_to_complex(area: &ComplexRect, pos: (u16, u16)) -> Complex {
    Complex {
        real: (pos.0 as f32 / SCREEN_WIDTH as f32) * (area.to_real - area.from_real)
            + area.from_real,
        imag: (1. - pos.1 as f32 / SCREEN_HEIGHT as f32) * (area.to_imag - area.from_imag)
            + area.from_imag,
    }
}

pub fn wait_till_released(k: u32) {
    while keyboard::scan().key_down(k) {}
}

pub fn keyboard_number<const N: usize>(num: &mut String<N>) -> Option<f32> {
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
        match num.chars().next() {
            Some('-') => *num = num.chars().skip(1).collect(),
            None | Some(_) => *num = num.chars().rev().chain(['-']).rev().collect(),
        }
    } else if keyboard_state.key_down(key::BACKSPACE) && num.len() > 0 {
        num.pop().unwrap();
    } else if keyboard_state.key_down(key::EXE) && !num.is_empty() {
        wait_till_released(key::EXE);
        return Some(num.as_str().parse::<f32>().unwrap());
    }
    None
}
