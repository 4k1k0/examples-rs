use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    process_file("./book.txt");
}

fn process_file(filename: &str) {
    let mut my_map = HashMap::<String, u32>::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            process_line(&mut my_map, line);
        }
    }

    println!("my map: {:?}", my_map);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_line(my_map: &mut HashMap<String, u32>, line: String) {
    if line == "" {
        return;
    }

    for part in line.split(" ") {
        let tmp = part.to_lowercase();
        if let Some(x) = my_map.get_mut(&tmp) {
            *x = *x + 1;
        } else {
            my_map.insert(String::from(&tmp), 1);
        }
    }
}
