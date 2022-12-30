use self::common::adder;

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