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
    // get standard input stream
    let input = io::stdin();

    // get input lines as iterative
    let mut lines = input.lock().lines().map(|_line| _line.ok().unwrap());

    // get one line at a time, until EOF
    while let Some(line) = lines.next() {
        // input should be like "5 4 3 2 1", this we want to turn into i32 array
        let mut arr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // sort the array
        quicksort(&mut arr);

        // print the sorted array
        println!(
            "{}",
            arr.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
