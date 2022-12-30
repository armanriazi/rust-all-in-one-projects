#![allow(dead_code, unused_variables, unused_imports)]


use pacman_lib::core::game::Game;
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
pub fn ff() -> Result<(), CustomError> {

    console.log("🦀 Rust + 🕸 Wasm = ❤");

    survey_init_env_logger(true);

    info!("Starting Up...");

    let gameObj = Game::new((5,5));

    gameObj.play("sample1");


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


//---


#[cfg(test)]
mod tests {
use super::*;
use pretty_assertions::assert_eq;
use pretty_assertions::{assert_eq, assert_ne};
    #[test]
    fn main(){
         let prefilled = Array2D::filled_with(42, 2, 3);
        assert_eq!(prefilled.num_rows(), 2);
        assert_eq!(prefilled.num_columns(), 3);
        assert_eq!(prefilled[(0, 0)], 42);

        // Create an array from the given rows. You can also use columns
        // with the `columns` function
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let from_rows = Array2D::from_rows(&rows)?;
        assert_eq!(from_rows.num_rows(), 2);
        assert_eq!(from_rows.num_columns(), 3);
        assert_eq!(from_rows[(1, 1)], 5);

        // Create an array from a flat Vec of elements in row major or
        // column major order.
        let column_major = vec![1, 4, 2, 5, 3, 6];
        let from_column_major =
            Array2D::from_column_major(&column_major, 2, 3)?;
        assert_eq!(from_column_major.num_rows(), 2);
        assert_eq!(from_column_major.num_columns(), 3);
        assert_eq!(from_column_major[(1, 1)], 5);

        // Implements `Eq` if the element type does.
        assert_eq!(from_rows, from_column_major);

        // Index into an array using a tuple of usize to access or alter
        // the array.
        let rows = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mut array = Array2D::from_rows(&rows)?;
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
        for element in array.column_iter(0)? {
            println!("{}", element);
        }

        // Iterate over all rows or columns.
        println!("All elements:");
        for row_iter in array.rows_iter() {
            for element in row_iter {
                print!("{} ", element);
            }
            println!();
        }
            
    }
}
