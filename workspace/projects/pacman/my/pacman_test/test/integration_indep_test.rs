use self::common::adder;

use array2d::Array2D;
/// process test
///
/// ## Commands
///
///
/// ```cargo test  -p rust-survey-json_lib -- --show-output```
///
/// ```cargo test  -p rust-survey-json_lib -- --show-output --ignore```
///             
/// ```cargo doc  --package rust-survey-json_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-survey-json_lib```
///
/// ## What
/// `A finder on json data for calculate the rate of users- there are 3 mode gain access to the json content"`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the uniqe empty type main function
///
/// # Return
/// 
/// `passed`
/// 

#[cfg(test)]

use pretty_assertions::assert_eq;
use proptest::prelude::*;
mod common;


#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}


#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
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