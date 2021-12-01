// Graphics library
extern crate minifb;
use minifb::{Window, WindowOptions, Key};

// Consts
use super::super::{HEIGHT, WIDTH, HEIGHT_F32, SIZE_F32};

/// # Visualizer
/// A structure that adds functionality to the window and updates buffer based on functions
pub struct Visualizer {
    window: Window,
    buffer: Vec<u32>
}

impl Visualizer {
    /// # `new`
    /// Initializes the window and buffer of the visualizer with default name and sizes.
    /// ## Parameters
    /// `update_rate_millies: Option<u64>` - The interval in milliseconds between each frame draw call. Passing `None` means default value, 0.
    pub fn new(update_rate_millis: Option<u64>) -> Visualizer {
        let mut vis = Visualizer {
            window: Window::new("Sorting Algorithms", WIDTH, HEIGHT, WindowOptions::default())
                            .expect("Error Creating Window!"),

            buffer: vec![0; WIDTH * HEIGHT]
        };

        // Update FPS based on given input
        vis.window.limit_update_rate(match update_rate_millis {
            Some(e) => Some(std::time::Duration::from_millis(e)),
            _ => Some(std::time::Duration::from_millis(0))
        });

        vis
    }

    /// # `paint`
    /// Paint the given array into the window and highlights a bar at the given index with the colour green.
    /// ## Parameters
    /// `array : &Vec<usize>` - The Array to draw on the screen
    /// `highlight: usize` - The index of the bar to highlight
    pub fn paint(&mut self, array: &Vec<usize>, highlight: usize) {

        // Set colours to 0 (Black)
        self.buffer.iter_mut().map(|x| *x = 0).count();

        for num in 0..array.len() {

            let val = array[num] as f32;

            // Height of bar is proportional the element's value. 
            // Looping through equal to the height, drawing pixels
            for magnitude in 0..((val / SIZE_F32) * HEIGHT_F32) as usize {
                
                let color = if num == highlight {0x00ff00} else {0xffffff};

                // Draw pixels from bottom up and two pixels wide
                self.buffer[WIDTH * (HEIGHT - magnitude - 1) + 2 * num] = color;
                self.buffer[WIDTH * (HEIGHT - magnitude - 1) + 2 * num + 1] = color;
            }
        }

        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();

        // If window is closed, shutdown program (avoids window closing but program running in background to finish sorting)
        if !self.window.is_open() { std::process::exit(0); }
    }

    /// # `end`
    /// Added to the end of the visualization process to avoid window and process closing down upong finished sorting
    pub fn end(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.update();
        }
    }
}



