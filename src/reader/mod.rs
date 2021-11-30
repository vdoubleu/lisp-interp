pub mod interp_args;

use std::io::{
    self, 
    BufReader, 
    BufRead
};
use std::fs;
use crate::reader::interp_args::InterpArgs;
use clap::{
    Arg,
    App,
};

pub fn read(interp_args: &InterpArgs) -> String {
    let inp = interp_args.file_name.clone();
    let reader: Box<dyn BufRead> = match inp {
        None => Box::new(BufReader::new(io::stdin())),
        Some(file_name) => Box::new(BufReader::new(fs::File::open(file_name).unwrap())),
    };
    
    let mut prog: String = "".to_string();
    for line in reader.lines() {
        match line {
            Ok(l) => prog += &(" ".to_string() + &l),
            Err(e) => panic!("Error: {}", e),
        }
    }

    return prog;
}

pub fn get_args() -> InterpArgs {
    let matches = App::new("FL Interp")
        .version("0.1")
        .about("Interpreter for Faux Lisp scripting language")
        .arg(Arg::new("no-interp")
             .about("disables AST interpreting")
             .long("no-interp")
             .short('n')
             .required(false))
        .arg(Arg::new("print-tokens")
            .about("prints program tokens")
            .long("print-tokens")
            .short('t')
            .required(false))
        .arg(Arg::new("lib-path")
            .about("path to stdlib root directory")
            .long("lib-path")
            .short('l')
            .required(false)
            .takes_value(true))
        .arg(Arg::new("<input-file>").required(false))
        .get_matches();


    return InterpArgs {
        no_interp: matches.is_present("no-interp"),
        print_tokens: matches.is_present("print-tokens"),
        file_name: matches.value_of("<input-file>").map(|s| s.to_string()),
        lib_path: if matches.is_present("lib-path") {
            matches.value_of("lib-path").unwrap().to_string()
        } else {
            String::from("")
        },
    };
}
