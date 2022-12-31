#![allow(dead_code, unused_variables, unused_imports)]

use pacman_lib::core::play::Play;
use pacman_lib::core::stateboard::{Message as MessageStateBoard };
use pacman_lib::core::statepacman::{Message as MessageStatePacman};
use pacman_lib::{core::error::CustomError};
use log::{debug, error, log_enabled, info, Level};
use env_logger::{Builder, Target};
use std::{env, fs, path::Path};
use std::{fs::File, env::args};
use std::io::BufReader;
use array2d::{Array2D, Error};


/// Main
///
/// ## Commands
///
///
/// ```RUST_LOG=INFO cargo run  -p pacman_bin --bin pacman_main_bin file /mnt/home/rust-all-in-one-projects/workspace/projects/pacman/my/pacman_lib/src/core/commands/text-sample-0.txt```
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
/// ## TODO
/// `is_xy_not_occupied in boardspace`
///



/// cargo run
/// RUST_LOG=INFO time cargo run pacman ../data/sample1.json

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn main() -> Result<(), CustomError> {

    init_app();
    
    survey_init_env_logger(true);

    info!("Starting Up...");

    let mut args: Vec<String> = env::args().collect();
    let mut file = String::default();

    if (&args).len() <= 1 {
        info!("** Please select a runner mode\n Help(file path transaction_list, or macrojson transaction_list)\n Default is cargo run macrojson **\n");
        args.push("macrojson".to_owned());
    } else {        
        file = (&args[2]).trim().to_lowercase();
    }

    let  mut play= Play{};
    let _stateboard=Play::arrange_stateboard(&(5_isize,5_isize));
    let mut _pacman=Play::arrange_statepacman(_stateboard);
    
    let _= Play::play(&file,&mut _pacman).unwrap();

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

fn init_app() -> impl std::process::Termination {
    println!("🦀 Rust + 🕸 Wasm = ❤");
    let machine_kind = if cfg!(unix) {
        println!("I'm running on a {} machine!", "unix");
        std::process::ExitCode::SUCCESS
    } else if cfg!(windows) {
        println!("I'm running on a {} machine!", "windows");
        std::process::ExitCode::SUCCESS
    } else {
        println!("I'm running on a {} machine!", "unknown");
        std::process::ExitCode::FAILURE
    };
}

