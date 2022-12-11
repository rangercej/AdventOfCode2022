use std::fs::File;
use std::io::BufReader;

pub fn get_inputfilereader() -> BufReader<File>
{
    let process_fullpath = std::env::current_exe().unwrap();
    let process_path = process_fullpath.parent().unwrap();

    let fpath = process_path.join("..\\..\\src\\input.txt");
    let fname = std::fs::canonicalize(fpath).unwrap();

    println!("Processing input from {}", fname.to_string_lossy());

    let file = match File::open(fname) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Failed to open file: {}", err);
            std::process::exit(1);
        }
    };

    BufReader::new(file)
}
