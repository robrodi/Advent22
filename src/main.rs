use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path ="/Users/rrodi/Documents/src/aoc22/inputs/1";
    let mut max = 0;
    let mut cur: i32 =0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if cur > max
                {
                    max = cur;
                }
               
                if ip.len() == 0
                {
                    cur = 0;
                }
                else
                {
                    cur += ip.parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("Max : {}.", max);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}