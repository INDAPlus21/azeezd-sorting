// === TESTS ===
// Tests are made on the same identical array
use super::algorithms;

#[test]
pub fn insertion_test() {
    let mut array : Vec<usize> = vec![8,7,6,9,5,4,2,1,3,0];
    algorithms::insertion::insertion_sort(&mut array);

    assert_eq!(array, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn selection_test() {
    let mut array : Vec<usize> = vec![8,7,6,9,5,4,2,1,3,0];
    algorithms::selection::selection_sort(&mut array);

    assert_eq!(array, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn merge_test() {
    let array : Vec<usize> = vec![8,7,6,9,5,4,2,1,3,0];
    let sorted = algorithms::merge::merge_sort(&array);

    assert_eq!(sorted, vec![0,1,2,3,4,5,6,7,8,9]);
}

#[test]
fn pigeonhole_test() {
    let mut array : Vec<usize> = vec![8,7,6,9,5,4,2,1,3,0];
    algorithms::pigeonhole::pigeonhole_sort(&mut array);

    assert_eq!(array, vec![0,1,2,3,4,5,6,7,8,9]);
}