#[allow(dead_code)]
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

