use super::super::{HEIGHT, WIDTH};
extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use super::super::painter;

pub fn insertion_sort(unsorted: &mut Vec<usize>)
{
    let mut i : usize = 1;
    let length = unsorted.len();
    
    while i < length {
        let x = unsorted[i];
        let mut j : isize = i as isize - 1;

        while j >= 0 && unsorted[j as usize] > x {
            unsorted[j as usize + 1] = unsorted[j as usize];
            j = j - 1;
        }

        unsorted[(j + 1) as usize] = x;
        i = i + 1;
    }
}

pub fn visual_insertion(unsorted: &mut Vec<usize>) {
    let mut buffer : Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Algorithms", WIDTH, HEIGHT, WindowOptions::default()).unwrap();


    window.limit_update_rate(Some(std::time::Duration::from_micros(10)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut i : usize = 1;
        let length = unsorted.len();
        
        while i < length {
            let x = unsorted[i];
            let mut j : isize = i as isize - 1;

            while j >= 0 && unsorted[j as usize] > x {
                unsorted[j as usize + 1] = unsorted[j as usize];
                j = j - 1;
                painter::paint(&unsorted, &mut window, &mut buffer, j as usize);
            }

            unsorted[(j + 1) as usize] = x;
            painter::paint(&unsorted, &mut window, &mut buffer, j as usize);
            i = i + 1;
        }
    }
}