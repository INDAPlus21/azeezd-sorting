mod visuals;
mod algorithms;

use rand::thread_rng;
use rand::seq::SliceRandom;

// Configure window
const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const HEIGHT_F32: f32 = HEIGHT as f32;

// Elements to be sorted
const SIZE: usize = WIDTH / 2;
const SIZE_F32: f32 = SIZE as f32;

fn main(){
    // Get algorithm name 
    println!("Algorithm? ");
    let mut buf : String = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");

    // Generate array and shuffle
    let mut array : Vec<usize> = (0..SIZE).collect();
    array.shuffle(&mut thread_rng());

    // Visualize according to input
    match buf.as_str().to_lowercase().trim() {
        "selection" => visuals::algorithm_visualiser::visual_selection(&mut array),
        "insertion" => visuals::algorithm_visualiser::visual_insertion(&mut array),
        "pigeonhole" => visuals::algorithm_visualiser::visual_pigeonhole(&mut array),
        "merge" => visuals::algorithm_visualiser::visual_merge(&mut array),

        _ => panic!("No such algorithm \"{}\" available to visualize!", buf.trim())
    }
}

#[cfg(test)]
mod unit_tests;