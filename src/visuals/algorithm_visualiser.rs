use super::painter::{Visualizer};

pub fn visual_insertion(unsorted: &mut Vec<usize>) {
    
    let mut vis = Visualizer::new(None);

    let mut i : usize = 1;
    let length = unsorted.len();
    
    while i < length {
        let x = unsorted[i];
        let mut j : isize = i as isize - 1;

        while j >= 0 && unsorted[j as usize] > x {
            unsorted[j as usize + 1] = unsorted[j as usize];
            j = j - 1;
            vis.paint(&unsorted, j as usize);
        }

        unsorted[(j + 1) as usize] = x;
        vis.paint(&unsorted, j as usize);
        i = i + 1;
    }

    vis.end();
}

pub fn visual_selection(unsorted: &mut Vec<usize>) {
    let mut vis = Visualizer::new(None);


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
            vis.paint(&unsorted, j as usize);
        }

        if min_index != i {
            let temp = unsorted[i];
            unsorted[i] = unsorted[min_index];
            unsorted[min_index] = temp;
        }

        i = i + 1;

    }

    vis.end();
}

pub fn visual_pigeonhole(unsorted: &mut Vec<usize>) {

    let mut vis = Visualizer::new(Some(1));

    let low = unsorted.iter().min().unwrap();

    let size = unsorted.iter().max().unwrap() - low + 1;

    let mut pigeonholes : Vec<Vec<usize>> = vec![vec![]; size];

    let mut i = 0;
    let a = unsorted.clone();
    for val in unsorted.into_iter() {
        pigeonholes[*val as usize].push(*val);
        vis.paint(&a, i as usize);
        i += 1;
    }

    let mut index: usize = 0;
    for hole in pigeonholes {
        for val in hole {
            unsorted[index as usize] = val;
            index += 1;
            vis.paint(&unsorted, index as usize);
        }
    }

    vis.end()
}