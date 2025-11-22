mod files;

fn main() {
    if let Err(e) = files::process_file("./book.txt") {
        eprintln!("error while processing file {}", e)
    }
}

