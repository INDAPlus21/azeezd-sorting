mod algorithms;
mod painter;
use rand::thread_rng;
use rand::seq::SliceRandom;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const SIZE: usize = WIDTH;

fn main(){
    println!("Algorithm? ");
    let mut buf : String = String::new();
    std::io::stdin().read_line(&mut buf).expect("Error");

    let mut array : Vec<usize> = (0..SIZE).collect();
    array.shuffle(&mut thread_rng());
    match buf.as_str().to_lowercase().trim() {
        "selection" => algorithms::selection::visual_selection(&mut array),
        "insertion" => algorithms::insertion::visual_insertion(&mut array),
        "pigeonhole" => algorithms::pigeonhole::visual_pigeonhole(&mut array),
        _ => ()
    }
}

#[cfg(test)]
mod unit_tests;