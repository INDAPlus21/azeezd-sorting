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