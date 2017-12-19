extern crate getopts;

use std::{env, process};
use getopts::Options;
// use std::fs::File;
// use std::path::Path;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}
/*
fn do_head(file: File, lines: i32) {
    
}
*/
fn main() {
    //argsを取得
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // optionの設定
    opts.optopt("n", "", "set number of lines", "LINES");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }
    let output = matches.opt_str("n");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return;
    };
    do_work(&input, output);
}
