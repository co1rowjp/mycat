extern crate getopts;
use getopts::Options;
use getopts::Matches;
use std::env;
use std::fs;
use std::io::{BufReader, BufRead, Read};
use std::io;
use std::collections::HashMap;

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

fn escape_char_lookup_table<'a>() -> HashMap<char, &'a str> {
    return [
        (0x00_u8.into(), "^@"), // NUL
        (0x01_u8.into(), "^A"), // SOH
        (0x02_u8.into(), "^B"), // STX
        (0x03_u8.into(), "^C"), // ETX
        (0x04_u8.into(), "^D"), // EOT
        (0x05_u8.into(), "^E"), // ENQ
        (0x06_u8.into(), "^F"), // ACK
        (0x07_u8.into(), "^G"), // BEL
        (0x08_u8.into(), "^H"), // BS
        // (0x09_u8.into(), "^I"), // HT
        // (0x0a_u8.into(), "^J"), // LF
        (0x0b_u8.into(), "^K"), // VT
        (0x0c_u8.into(), "^L"), // FF
        (0x0d_u8.into(), "^M"), // CR
        (0x0e_u8.into(), "^N"), // SO
        (0x0f_u8.into(), "^O"), // SI
        (0x10_u8.into(), "^P"), // DLE
        (0x11_u8.into(), "^Q"), // DC1
        (0x12_u8.into(), "^R"), // DC2
        (0x13_u8.into(), "^S"), // DC3
        (0x14_u8.into(), "^T"), // DC4
        (0x15_u8.into(), "^U"), // NAK
        (0x16_u8.into(), "^V"), // SYN
        (0x17_u8.into(), "^W"), // ETB
        (0x18_u8.into(), "^X"), // CAN
        (0x19_u8.into(), "^Y"), // EM
        (0x1a_u8.into(), "^Z"), // sub
        (0x1b_u8.into(), "^["), // esc
        (0x1c_u8.into(), "^\\"), // fs
        (0x1d_u8.into(), "^]"), // gs
        (0x1e_u8.into(), "^^"), // rs
        (0x1f_u8.into(), "^_"), // us
        (0x7f_u8.into(), "^?"), // del
    ].iter().cloned().collect();
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

fn replace_nonprinting(line: String, lookup_table: & HashMap<char, &str>) -> String {
    let mut vec = Vec::new();
    for c in line.chars() {
        match lookup_table.get(&c) {
            Some(r) => for cc in r.chars() {
                vec.push(cc);
            },
            None => vec.push(c),
        }
    }
    return vec.iter().cloned().collect::<String>();
}

fn print_from_buffer<R: Read>(mut buffer: BufReader<R>, matches: &Matches) -> Result<(), String> {
    let show_ends = matches.opt_present("E") || matches.opt_present("e") || matches.opt_present("A");
    let show_line_number = matches.opt_present("n");
    let show_line_number_non_blank = matches.opt_present("b");
    let show_tabs = matches.opt_present("T") || matches.opt_present("A") || matches.opt_present("t");
    let squeeze_blank = matches.opt_present("s");
    let show_nonprinting = matches.opt_present("v") || matches.opt_present("A") || matches.opt_present("e") ||  matches.opt_present("t");
    let lookup_table = escape_char_lookup_table();
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

        if show_nonprinting {
            l = replace_nonprinting(l, &lookup_table);
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
