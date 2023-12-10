use std::path::{Path, PathBuf};

pub mod map;
pub use num;

/// Read the input file for the given year and day.
/// Will look for the file in the directory specified by the AOC_PATH environment variable
/// or fall back to ../aoc_input
/// Current year will be appended to the path.
/// The file is expected to be named dayXX.txt where XX is the day number.
pub fn read_input_file(year: u16, day: u8) -> String {
    let path = assemble_file_path(year, day);
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Error reading file: {} from path: {}", e, &path.display()))
        .unwrap()
}

fn assemble_file_path(year: u16, day: u8) -> PathBuf {
    let path = std::env::var("AOC_PATH").unwrap_or("../aoc_input".to_string());
    Path::new(&path)
        .join(year.to_string())
        .join(format!("day{:02}.txt", day))
}
