use sudoku::Sudoku;
use std::time::{Instant};

fn main() {
    let start = Instant::now();
    // create a unique Sudoku puzzle...
    let generated = Sudoku::generate_unique();
    let line = generated.to_str_line();
    let sudoku_line: &str = &line;

    // print the puzzle string...
    println!("Puzzle: {} ns", start.elapsed().as_nanos());
    println!("{}", sudoku_line);

    // generate a new sudoku from the string...
    let sudoku = Sudoku::from_str_line(sudoku_line).unwrap();
    let solved = Instant::now();

    // Solve, print or convert the sudoku to another format
    if let Some(solution) = sudoku.solve_unique() {
        let elapsed = solved.elapsed().as_nanos();
        // print the solution in line format
        println!("Solution: {} ns", elapsed);
        println!("{}", solution);

        // or return it as a byte array
        let _cell_contents: [u8; 81] = solution.to_bytes();
    }
}