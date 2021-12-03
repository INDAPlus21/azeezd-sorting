use super::painter::{Visualizer};

/// # `visual_insertion`
/// Visualize Insertion Sort
/// ## Parameters
/// `unsorted: &mut Vec<usize>` - The array to sort
pub fn visual_insertion(unsorted: &mut Vec<usize>) {
    
    // Initialize visualizer
    let mut vis = Visualizer::new(None);

    let mut i : usize = 1;
    let length = unsorted.len();
    
    while i < length {
        let x = unsorted[i];
        let mut j : isize = i as isize - 1;

        while j >= 0 && unsorted[j as usize] > x {
            unsorted[j as usize + 1] = unsorted[j as usize];
            j = j - 1;
            vis.paint(&unsorted, j as usize); // Draw
        }

        unsorted[(j + 1) as usize] = x;
        vis.paint(&unsorted, j as usize); // Draw
        i = i + 1;
    }

    vis.end();
}

/// # `visual_selection`
/// Visualize Selection Sort
/// ## Parameters
/// `unsorted: &mut Vec<usize>` - The array to sort
pub fn visual_selection(unsorted: &mut Vec<usize>) {
    // Initialize visualizer
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
            vis.paint(&unsorted, j as usize); // Draw
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

/// # `visual_merge`
/// Visualize Merge Sort
/// ## Parameters
/// `unsorted: &Vec<usize>` - The array to sort
pub fn visual_merge(unsorted: &Vec<usize>) {
    // Initialize visualizer 
    let mut vis = Visualizer::new(Some(3));

    // Start merge sort visualization
    merge_sort(unsorted, &mut vis, 0, &mut unsorted.clone());
    vis.end();
}

/// # `merge_sort`
/// Starts merge sorting by taking the array, visualizer and other data
/// 
/// ## Parameters
/// `unsorted: &Vec<usize>` - (Sub)array to sort
/// `visualizer: &mut Visualizer` - Visualizer structure
/// `low_index: usize` - Starting index of the subarray in the total vector
/// `visualizable_vec: &mut Vec<usize>` - The vector that has the subarrays placed in. Used for visual purposed only
/// 
/// ## Returns
/// `Vec<usize>` - The merged and sorted (sub)array
fn merge_sort(unsorted: &Vec<usize>, visualizer: &mut Visualizer, low_index: usize, visualizable_vec: &mut Vec<usize>) -> Vec<usize> {

    // Return single element
    if unsorted.len() == 1 { return unsorted.to_owned(); }

    // Split (sub)array
    let left = &unsorted[0..unsorted.len() / 2];
    let right = &unsorted[unsorted.len() / 2..];

    // Recursion
    let mut left : Vec<usize> = merge_sort(&left.to_vec(), visualizer, low_index, visualizable_vec);
    let mut right : Vec<usize> = merge_sort(&right.to_vec(), visualizer, low_index + unsorted.len() / 2 + 1, visualizable_vec);

    return merge(&mut left, &mut right, visualizer, low_index, visualizable_vec);
}

/// # `merge`
/// Sort-merges two subarrays
/// 
/// ## Parameters
/// `vec1: &mut Vec<usize>` - Subarray 1
/// `vec2: &mut Vec<usize>` - Subarray 2
/// `visualizer: &mut Visualizer` - Visualizer structure
/// `low_index: usize` - Starting index of the subarray in the total vector
/// `visualizable_vec: &mut Vec<usize>` - The vector that has the subarrays placed in. Used for visual purposed only
/// 
/// ## Returns
/// `Vec<usize>` - The merged and sorted (sub)array
fn merge(vec1: &mut Vec<usize>, vec2: &mut Vec<usize>, visualizer: &mut Visualizer, low_index: usize, visualizable_vec: &mut Vec<usize>) -> Vec<usize>
{
    // Vec where both are merged. Size is already known
    let mut merged : Vec<usize> = Vec::with_capacity(vec1.len() + vec2.len());

    // Merging
    while vec1.len() > 0 && vec2.len() > 0 {
        if vec1[0] > vec2[0] {
            merged.push(vec2[0]);
            vec2.remove(0);
        }
        else {
            merged.push(vec1[0]);
            vec1.remove(0);
        }
        display_merging(visualizer, low_index, &merged, visualizable_vec);
    }


    // Put rest
    while vec1.len() > 0 {
        merged.push(vec1[0]);
        vec1.remove(0);
    }
    display_merging(visualizer, low_index, &merged, visualizable_vec);

    while vec2.len() > 0 {
        merged.push(vec2[0]);
        vec2.remove(0);
    }
    display_merging(visualizer, low_index, &merged, visualizable_vec);

    return merged;
}

/// # `display_merging`
/// Displays the merging process (special function only for merge sort)
/// ## Parameters
/// `visualizer: &mut Visualizer` - Visualizer structure
/// `low_index: usize` - Starting index of the subarray in the total vector
/// `merged: &Vec<usize>` - The merged subarray that will be displayed and put in the visualizable
/// `visualizable_vec: &mut Vec<usize>` - The vector that has the subarrays placed in. Used for visual purposed only
/// 
/// ## Returns
/// `Vec<usize>` - The merged and sorted (sub)array
fn display_merging(visualizer: &mut Visualizer, low_index: usize, merged: &Vec<usize>, visualizable_vec: &mut Vec<usize>) {
    for idx in 0..merged.len() {
        // Place subarray in its place in the complete vector
        if let Some(e) = visualizable_vec.get_mut(low_index + idx) {
            *e = *merged.get(idx).unwrap();
        }
    }
    visualizer.paint(&visualizable_vec, low_index);
}

/// # `visual_pigeonhole`
/// Visualize Pigeonhole Sort
/// ## Parameters
/// `unsorted: &mut Vec<usize>` - The array to sort
pub fn visual_pigeonhole(unsorted: &mut Vec<usize>) {

    // Initialize visualizer. Update rate set to 3 milliseconds because it's hella fast
    let mut vis = Visualizer::new(Some(3));

    // Initialize boundries
    let low = unsorted.iter().min().unwrap();
    let size = unsorted.iter().max().unwrap() - low + 1;

    // Prepare pigeonholes
    let mut pigeonholes : Vec<Vec<usize>> = vec![vec![]; size];

    let mut index = 0;

    // Cloning vector to avoid ownership conflicts
    let vec_clone = unsorted.clone();

    // Place numbers in their pigeonholes
    for val in unsorted.into_iter() {
        pigeonholes[*val as usize].push(*val);
        vis.paint(&vec_clone, index as usize);
        index += 1;
    }

    // Place number from pigeonholes back to vector
    let mut index = 0;
    for hole in pigeonholes {
        for val in hole {
            unsorted[index as usize] = val;
            index += 1;
            vis.paint(&unsorted, index as usize);
        }
    }

    vis.end()
}