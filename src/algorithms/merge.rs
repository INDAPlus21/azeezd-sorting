#[allow(dead_code)]
pub fn merge_sort(unsorted: &Vec<usize>) -> Vec<usize> {
    if unsorted.len() == 1 { return unsorted.to_owned(); }

    let left = &unsorted[0..unsorted.len() / 2];
    let right = &unsorted[unsorted.len() / 2..];
    let mut left : Vec<usize> = merge_sort(&left.to_vec());
    let mut right : Vec<usize> = merge_sort(&right.to_vec());

    return merge(&mut left, &mut right);
}

fn merge(vec1: &mut Vec<usize>, vec2: &mut Vec<usize>) -> Vec<usize>
{
    let mut merged : Vec<usize> = Vec::with_capacity(vec1.len() + vec2.len());

    while vec1.len() > 0 && vec2.len() > 0 {
        if vec1[0] > vec2[0] {
            merged.push(vec2[0]);
            vec2.remove(0);
        }
        else {
            merged.push(vec1[0]);
            vec1.remove(0);
        }
    }

    while vec1.len() > 0 {
        merged.push(vec1[0]);
        vec1.remove(0);
    }

    while vec2.len() > 0 {
        merged.push(vec2[0]);
        vec2.remove(0);
    }

    return merged;
}