use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Err(e) = process_file("./book.txt") {
        eprintln!("error while processing file {}", e)
    }
}

fn process_file(filename: &str) -> io::Result<()> {
    let mut my_map = HashMap::<String, u32>::new();

    let lines = read_lines(filename)?;
    for line in lines.flatten() {
        process_line(&mut my_map, &line);
    }

    println!("my map: {:?}", my_map);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_line(my_map: &mut HashMap<String, u32>, line: &str) {
    if line.trim().is_empty() {
        return;
    }

    for part in line.split_whitespace() {
        let tmp = part.to_lowercase();
        my_map
            .entry(tmp)
            .and_modify(|counter| *counter +=1)
            .or_insert(1);
    }
}
