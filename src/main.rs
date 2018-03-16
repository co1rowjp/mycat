extern crate getopts;
use getopts::Options;
use getopts::Matches;
use std::env;
use std::fs;
use std::io::{BufReader, BufRead, Read};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn make_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("A", "show-all", "equivalent to -vET");
    opts.optflag("b", "number-nonblank", "number nonblank output lines");
    opts.optflag("e", "", "equivalent to -vE");
    opts.optflag("E", "show-ends", " display $ at end of each line");
    opts.optflag("n", "", "number all output lines");
    opts.optflag("s", "squeeze-blank", "never more than one single blank line");
    opts.optflag("t", "", "equivalent to -vT");
    opts.optflag("T", "show-tabs", "display TAB characters as ^I");
    opts.optflag("u", "", "(ignored)");
    opts.optflag("v", "show-nonprinting", "use ^ and M- notation, except for LFD and TAB");
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");
    return opts;
}

fn print_file_contents(path: &str, _matches: &Matches) -> Result<(), String> {
    let file = try!(fs::File::open(path).map_err(|e| e.to_string()));
    let mut buffer = BufReader::new(file);
    for (index, line) in buffer.by_ref().lines().enumerate() {
        let l =  try!(line.map_err(|e| e.to_string()));
        println!("{}", l);
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = make_options();

    let mut matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() == 0 {
        println!("not implemented.");
        return;
    }

    for filePath in &matches.free {
        print_file_contents(&filePath, &matches).map_err(|e| print!("{}", e));
    }

    return;
}
