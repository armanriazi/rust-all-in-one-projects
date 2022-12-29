#![allow(dead_code, unused_variables)]
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};
use process_line_lib::process_lines;
/// Main
///
/// ## Commands
/// 
/// ```cargo run  -- arman  -- -```
///
///```cargo run  -- arman  -- readme.txt```
///
/// ```cargo run -q -p process_line_bin --bin process_line_main_bin -- arman  -- /mnt/home/rust-all-in-one-projects/workspace/projects/process_line/rust-in-action/process_line_bin/readme.txt```
///
/// ```cargo doc  --package process_line_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package process_line_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// // ```compile_fail,ignore
///


 fn main() {
 let args = App::new("grep-lite")
                .version("0.1")
                .about("searches for patterns")
                .arg(Arg::with_name("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true))
                .arg(Arg::with_name("input")
                .help("File to search")
                .takes_value(true)
                .required(false))
                .get_matches();

 let pattern = args.value_of("pattern").unwrap();
 let re = Regex::new(pattern).unwrap();

 let input = args.value_of("input").unwrap_or("-");

 if input == "-" {
      let stdin = io::stdin();
      let reader = stdin.lock();
      process_lines(reader, re);
    } else {
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    process_lines(reader, re);
    }
 }
