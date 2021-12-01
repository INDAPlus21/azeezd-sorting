# Azeez Daoud - Sorting Algorithms

## How to Visualize
Type `cargo run --release` (release mode is faster than debug)

You will be prompted with `Algorithm? ` to which you can type:
- `insertion`
- `selection`
- `merge`
- `pigeonhole`

to visualize the algorithm
## Supported Visualised Algorithms
- Insertion Sort
- Selection Sort
- Merge Sort
- Pigeonhole Sort

## Algorithm of my Choice - Pigeonhole Sort
The Pigeonhole Sort starts by creating "pigeonholes" to place the numbers in. The amount of pigeonholes is equal to the difference between highest and lowest number (RIP Memory if boundries of `usize` are in the array). Going through the unsorted array, every number is placed in their respective index in the pigeonhole vector. Then going through the pigeonholes in order, place the numbers in each pigeonhole back into the array in order.  