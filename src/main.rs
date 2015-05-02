#![feature(slice_patterns)]
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

fn main() {
    // and_then: great when your value is an option or a result
    // map: great when your value is concrete
    let num: Option<u32> = std::env::args().nth(1).and_then(|n| n.parse().ok());
    let file: Option<BufReader<File>> = std::env::args().nth(2)
        .and_then(|path| File::open(&Path::new(&path)).ok())
        .map(|file| BufReader::new(file));

    match (num, file) {
        (Some(n), Some(mut file)) => print_result(n, &mut file),
        _ => println!("Something when terribly, horribly wrong."),
    };
}

fn print_result<R: BufRead>(n: u32, f: R) {
    println!("{}", f.lines()
        .filter_map(|l| l.ok().and_then(|l| l.parse::<u32>().ok()))
        .map(|x| n * x)
        .fold(0, |a, b| a + b));
}
