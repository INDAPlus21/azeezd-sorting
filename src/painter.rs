extern crate minifb;
use minifb::{Window};

use super::{HEIGHT, WIDTH, SIZE};

pub fn paint(array: &Vec<usize>, window: &mut Window, buffer: &mut Vec<u32>, highlight: usize) {
    buffer.iter_mut().map(|x| *x = 0).count();
    for num in 0..array.len() {
        for magnitude in 0..((array[num] as f32 / SIZE as f32) * HEIGHT as f32) as usize {
            buffer[WIDTH * magnitude + num] = if highlight == num {0x00ff00} else {0xffffff};
        }
    }
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
}