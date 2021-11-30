use super::super::{HEIGHT, WIDTH};
extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use super::super::painter;

pub fn selection_sort(unsorted: &mut Vec<usize>) {
    let mut i : usize = 0;
    let length = unsorted.len();

    while i < length - 1
    {
        let mut min_index = i;
        let mut j = i + 1;

        while j < length {
            if unsorted[j] < unsorted[min_index] {
                min_index = j;
            }
            j = j + 1;
        }

        if min_index != i {
            let temp = unsorted[i];
            unsorted[i] = unsorted[min_index];
            unsorted[min_index] = temp;
        }

        i = i + 1;
    }
}

pub fn visual_selection(unsorted: &mut Vec<usize>) {
    let mut buffer : Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Algorithms", WIDTH, HEIGHT, WindowOptions::default()).unwrap();


    window.limit_update_rate(Some(std::time::Duration::from_micros(10)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut i : usize = 0;
        let length = unsorted.len();

        while i < length - 1
        {
            let mut min_index = i;
            let mut j = i + 1;

            while j < length {
                if unsorted[j] < unsorted[min_index] {
                    min_index = j;
                }
                j = j + 1;
                painter::paint(&unsorted, &mut window, &mut buffer, j as usize);
            }

            if min_index != i {
                let temp = unsorted[i];
                unsorted[i] = unsorted[min_index];
                unsorted[min_index] = temp;
            }

            i = i + 1;
        }
    }
}