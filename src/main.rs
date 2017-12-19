extern crate getopts;

use std::{env, process};
use getopts::Options;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

/*
fn do_work(inp: &str, lines: Option<String>, filename: Option<String>) {
    println!("{}", inp);
    match lines {
        Some(x) => println!("{}", x),
        None => println!("No lines"),
    }
    match filename {
        Some(x) => println!("{}", x),
        None => println!("No File"),
    }
}
*/

fn head(lines: i32, filename: String) -> i32 {
    // ファイル読み込み
    let string = String::from(filename);
    let path = Path::new(&string);
    let mut file = BufReader::new(File::open(&path).unwrap());

    if lines <= 0 {
        return 0;
    } else {
        let mut buf = String::new();
        let mut count = 0;

        let mut done = false;
        while !done {
            file.read_line(&mut buf).expect(
                "reading from cursor won't fail",
            );
            print!("{}", buf);
            count += 1;
            if lines == count || buf == "" {
                done = true;
            }
            buf.clear();
        }
        return 0;
    }
}

fn main() {
    //argsを取得
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let filename = args[4].clone();

    // optionの設定
    let mut opts = Options::new();
    opts.optopt("n", "", "set number of lines", "LINES");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }
    // let lines = matches.opt_str("n");
    let lines_string = match matches.opt_str("n") {
        Some(x) => x,
        None => process::exit(0),
    };
    let lines_i32 = lines_string.parse::<i32>().unwrap();
    /*
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, &opts);
        return;
    };
    */

    process::exit(head(lines_i32, filename));
}
