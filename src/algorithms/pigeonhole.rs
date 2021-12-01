#[allow(dead_code)]
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