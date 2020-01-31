use std::fs;
use std::path::Path;

extern crate clap;
use clap::{Arg, App};

fn main() {

    let matches = App::new("Dir Scan")
        .version("1.0.0")
        .author("Magdi . <migzzawi@gmail.com>")
        .about("dir scan")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("DIR")
            .help("Sets the input dir path to use")
            .required(true)
            .index(1))
    // .arg(Arg::with_name("OUTPUT")
    //         .help("Sets the output file to use")
    //         .required(true)
    //         .index(2))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    let input_path_dir:&str = matches.value_of("DIR").unwrap();

    let path = Path::new(input_path_dir);

    let (dirs, files) = get_dir_files(&path);

    println!("Dirs : {:?}", dirs);
    println!("Files : {:?}", files);
}


fn get_dir_files(path: &Path) -> (Vec<String>, Vec<String>){

    let paths = fs::read_dir(path).unwrap();

    let mut dirs :Vec<String> = Vec::new();
    let mut files :Vec<String> = Vec::new();

    for p in paths {
        let path = p.unwrap().path();
        if path.is_dir() {
            dirs.push(path.display().to_string());
        } else {
            files.push(path.display().to_string());
        }
    }

    (dirs, files)
}