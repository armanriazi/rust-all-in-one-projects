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
/// ## Example
/// `nothing`
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

/// ## Result
/// ```compile_fail,no_run
/// running 1 test
/// First column:
/// 1
/// 4
/// All elements:
/// 1 2 3 
/// 4 100 6 
/// test test ... ok
/// ```
#[test]
fn test(){
        let prefilled = Array2D::filled_with(42, 2, 3);
    assert_eq!(prefilled.num_rows(), 2);
    assert_eq!(prefilled.num_columns(), 3);
    assert_eq!(prefilled[(0, 0)], 42);

    // Create an array from the given rows. You can also use columns
    // with the `columns` function
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let from_rows = Array2D::from_rows(&rows).unwrap();
    assert_eq!(from_rows.num_rows(), 2);
    assert_eq!(from_rows.num_columns(), 3);
    assert_eq!(from_rows[(1, 1)], 5);

    // Create an array from a flat Vec of elements in row major or
    // column major order.
    let column_major = vec![1, 4, 2, 5, 3, 6];
    let from_column_major =
        Array2D::from_column_major(&column_major, 2, 3).unwrap();
    assert_eq!(from_column_major.num_rows(), 2);
    assert_eq!(from_column_major.num_columns(), 3);
    assert_eq!(from_column_major[(1, 1)], 5);

    // Implements `Eq` if the element type does.
    assert_eq!(from_rows, from_column_major);

    // Index into an array using a tuple of usize to access or alter
    // the array.
    let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let mut array = Array2D::from_rows(&rows).unwrap();
    array[(1, 1)] = 100;

    // Convert the array back into a nested Vec using `as_rows` or
    // `as_columns`.
    let array_rows = array.as_rows();
    assert_eq!(array_rows, vec![vec![1, 2, 3], vec![4, 100, 6]]);

    // Convert the array back into a flat Vec using `as_row_major` or
    // `as_column_major`.
    let array_column_major = array.as_column_major();
    assert_eq!(array_column_major, vec![1, 4, 2, 100, 3, 6]);

    // Iterate over a single row or column
    println!("First column:");
    for element in array.column_iter(0).unwrap() {
        println!("{:?}", element);
    }

    // Iterate over all rows or columns.
    println!("All elements:");
    for row_iter in array.rows_iter() {
        for element in row_iter {
            print!("{:?} ", element);
        }
        println!();
    }
        
}