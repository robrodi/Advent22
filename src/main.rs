use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn group_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut cur: i32 =0;

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        if let Ok(ip) = line {
            
            if ip.len() == 0
            {
                v.push(cur);
                cur = 0;
            }
            else
            {
                cur += ip.parse::<i32>().unwrap();
            }
        }
    }
    v.push(cur);
    v
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_input_lines(1) {
        let mut v = group_lines(lines);
        v.sort();
        
        println!("the top 1 is: {:?}", &v[v.len()-1..]);
      
        let top3 = &v[v.len()-3..];
        let sum: i32 = top3.iter().sum();
      
        println!("the total sum of the top3 is: {}", sum);
    }

    
}

fn read_input_lines(x: i32) -> io::Result<io::Lines<io::BufReader<File>>> {
    let folder ="/Users/rrodi/Documents/src/aoc22/inputs/";
    let filename= format!("{}{}", folder, x);
    read_lines(filename)
}
    // The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}