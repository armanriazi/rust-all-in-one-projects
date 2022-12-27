#![allow(dead_code, unused_variables)]
use pacman_lib::{core::error::CustomError};
use log::{debug, error, log_enabled, info, Level};
use env_logger::{Builder, Target};
use std::{env, fs, path::Path};
use std::{fs::File, env::args};
use std::io::BufReader;


/// Main
///
/// ## Commands
///
///
/// ```RUST_LOG=INFO cargo run  -p pacman_bin --bin pacman_main_bin file workspace/projects/pacman/my/pacman_bin/data/1.json```
///
/// ```cargo doc  --package pacman_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pacman_bin```
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
/// `nothing`
///



/// cargo run
/// RUST_LOG=INFO time cargo run pacman ../data/1.json

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn main() -> Result<(), CustomError> {


    survey_init_env_logger(true);

    info!("Starting Up...");



    Ok(())
}



pub fn survey_init_env_logger(is_enable:bool) {

    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    if is_enable{
       builder.init();
    }

    if log_enabled!(Level::Info) {
        info!("------------Welcome to env_logger------------");
    }
    else  {
        println!("-------env_logger have not been activated-------");
    }
}
