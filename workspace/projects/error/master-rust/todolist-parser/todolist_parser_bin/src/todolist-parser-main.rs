extern crate todolist_parser_lib;

use todolist_parser_lib::TodoList;

/// Main
///
/// ## Commands
///
///```RUST_BACKTRACE=1 cargo run  -p todolist_parser_bin --bin todolist-parser-main```
///
/// ```cargo run -q -p todolist_parser_bin --bin process_line_main_bin```
///
/// ```cargo doc  --package todolist_parser_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package todolist_parser_bin```
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
/// `TODO`
/// // ```rust, compile_fail,ignore
///



fn main() {
    let todos = TodoList::get_todos("../todo");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}", e.description());
            println!("{:?}", e)
        }
    }
}
