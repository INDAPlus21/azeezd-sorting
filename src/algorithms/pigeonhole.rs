use super::super::{HEIGHT, WIDTH};
extern crate minifb;
use minifb::{Key, Window, WindowOptions};
use super::super::painter;

pub fn pigeonhole_sort(unsorted: &mut Vec<usize>) {
    let low = unsorted.iter().min().unwrap();

    let size = unsorted.iter().max().unwrap() - low + 1;

    let mut pigeonholes : Vec<Vec<usize>> = vec![vec![]; size];

    for val in unsorted.into_iter() {
        pigeonholes[*val as usize].push(*val);
    }


    let mut index: usize = 0;
    for hole in pigeonholes {
        for val in hole {
            unsorted[index as usize] = val;
            index += 1;
        }
    }
}

pub fn visual_pigeonhole(unsorted: &mut Vec<usize>) {
    let mut buffer : Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Algorithms", WIDTH, HEIGHT, WindowOptions::default()).unwrap();


    window.limit_update_rate(Some(std::time::Duration::from_millis(2)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let low = unsorted.iter().min().unwrap();

        let size = unsorted.iter().max().unwrap() - low + 1;

        let mut pigeonholes : Vec<Vec<usize>> = vec![vec![]; size];

        let mut i = 0;
        let a = unsorted.clone();
        for val in unsorted.into_iter() {
            pigeonholes[*val as usize].push(*val);
            painter::paint(&a, &mut window, &mut buffer, i as usize);
            i += 1;
        }

        let mut index: usize = 0;
        for hole in pigeonholes {
            for val in hole {
                unsorted[index as usize] = val;
                index += 1;
                painter::paint(&unsorted, &mut window, &mut buffer, index as usize);
            }
        }
    }
}