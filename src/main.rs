extern crate getopts;
use getopts::Options;
use getopts::Matches;
use std::env;
use std::fs;
use std::io::{BufReader, BufRead, Read};
use std::io;

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
    //opts.optflag("v", "show-nonprinting", "use ^ and M- notation, except for LFD and TAB");
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");
    return opts;
}

fn print_file_contents(path: &str, matches: &Matches) -> Result<(), String> {
    let file = try!(fs::File::open(path).map_err(|e| e.to_string()));
    let buffer = BufReader::new(file);
    return print_from_buffer(buffer, matches);
}

fn print_stdin(matches: &Matches) -> Result<(), String> {
    let stdin = io::stdin();
    let buffer = BufReader::new(stdin);
    return print_from_buffer(buffer, matches);
}

fn print_from_buffer<R: Read>(mut buffer: BufReader<R>, matches: &Matches) -> Result<(), String> {
    let show_ends = matches.opt_present("E") || matches.opt_present("e") || matches.opt_present("A");
    let show_line_number = matches.opt_present("n");
    let show_line_number_non_blank = matches.opt_present("b");
    let show_tabs = matches.opt_present("T") || matches.opt_present("A") || matches.opt_present("t");
    let squeeze_blank = matches.opt_present("s");
    let mut prev_is_blank = false;
    let mut line_number = 1;

    for line in buffer.by_ref().lines() {
        let mut l =  try!(line.map_err(|e| e.to_string()));
        let current_is_blank = l.len() == 0;

        if squeeze_blank && prev_is_blank && current_is_blank {
            continue;
        }

        if show_line_number_non_blank {
            if !current_is_blank {
                l = format!("{:width$}  {}", line_number, l, width=5);
                line_number = line_number + 1;
            }
        } else if show_line_number {
            l = format!("{:width$}  {}", line_number, l, width=5);
            line_number = line_number + 1;
        }

        if show_ends {
            l = format!("{}$", l);
        }

        if show_tabs {
            l = l.replace("\t", "^I");
        }

        println!("{}", l);
        prev_is_blank = current_is_blank;
    }
    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = make_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("version") {
        println!("mycat version 0.01");
        println!("written by co1row");
        return;
    }

    if matches.free.len() == 0 {
        match print_stdin(&matches) {
            Ok(()) => (),
            Err(e) => print!("{}", e),
        }
        return;
    }

    for file_path in &matches.free {
        match print_file_contents(&file_path, &matches) {
            Ok(()) => (),
            Err(e) => print!("{}", e),
        }
    }

    return;
}
