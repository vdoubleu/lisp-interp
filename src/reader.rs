use std::io::{
    self, 
    BufReader, 
    BufRead
};
use std::fs;

pub fn read(args: &Vec<String>) -> String {
    let inp = args.iter().nth(1);
    let reader: Box<dyn BufRead> = match inp {
        None => Box::new(BufReader::new(io::stdin())),
        Some(file_name) => Box::new(BufReader::new(fs::File::open(file_name).unwrap())),
    };

    if args.len() >= 3 {
        println!("Warning: Currently all programs must be in\
                 the same file, only the first file will be run");
    }
    
    let mut prog: String = "".to_string();
    for line in reader.lines() {
        match line {
            Ok(l) => prog += &(" ".to_string() + &l),
            Err(e) => panic!("Error: {}", e),
        }
    }

    return prog;
}
