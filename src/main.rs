/***
 * Quicksort
 *
 * Author: Vilhelm Prytz <vilhelm@prytznet.se>
 *
 * Based on this template for I/O to Kattis
 * https://github.com/IndaPlus22/AssignmentInstructions-BlueNote/blob/main/task-17/kattis_template/src/main.rs
 */

mod qsort;

use std::io;
use std::io::prelude::*;

use qsort::quicksort;

/// Kattis calls main function to run your solution.
fn main() {
    let mut line = String::new();
    io::stdin().lock().read_to_string(&mut line).unwrap();

    let mut arr: Vec<i32> = line
        .split_whitespace()
        .skip(1) // first integer is the length of the array, which we can safely ignore
        .map(|s| s.parse().unwrap())
        .collect();

    quicksort(&mut arr);

    println!(
        "{}",
        arr.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
