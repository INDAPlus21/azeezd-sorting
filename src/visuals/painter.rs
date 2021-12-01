extern crate minifb;
use minifb::{Window, WindowOptions, Key};

use super::super::{HEIGHT, WIDTH, SIZE};

pub struct Visualizer {
    window: Window,
    buffer: Vec<u32>
}

impl Visualizer {
    pub fn new(update_rate_millis: Option<u64>) -> Visualizer {
        let mut vis = Visualizer {
            window: Window::new("Algorithms", WIDTH, HEIGHT, WindowOptions::default()).expect("Error Creating Window!"),
            buffer: vec![0; WIDTH * HEIGHT]
        };
        vis.window.limit_update_rate(match update_rate_millis {
            Some(e) => Some(std::time::Duration::from_millis(e)),
            _ => Some(std::time::Duration::from_millis(0))
        });

        vis
    }

    pub fn paint(&mut self, array: &Vec<usize>, highlight: usize) {
        self.buffer.iter_mut().map(|x| *x = 0).count();
        for num in 0..array.len() {
            for magnitude in 0..((array[num] as f32 / SIZE as f32) * HEIGHT as f32) as usize {
                self.buffer[WIDTH * (HEIGHT - magnitude - 1) + 2 * num] = if highlight == num {0x00ff00} else {0xffffff};
                self.buffer[WIDTH * (HEIGHT - magnitude - 1) + 2 * num + 1] = if highlight == num {0x00ff00} else {0xffffff};
            }
        }
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();

        if !self.window.is_open() { std::process::exit(0); }
    }

    pub fn end(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update();
        }
    }
}



