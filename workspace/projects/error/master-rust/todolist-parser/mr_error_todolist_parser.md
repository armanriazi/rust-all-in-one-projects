
To see the custom errors, try running the `basics.rs` example by initializing `TodoList`
in one of the following ways:

* `let todos = TodoList::get_todos("examples/malformed_todo");` - The case where we have a malformed todo.
* `let todos = TodoList::get_todos("examples/todos");` - The case where our `todos` file doesn't exist.


## thiserror Vs Anyhow
Use [thiserror](https://github.com/dtolnay/thiserror) if you care about designing your own dedicated error type(s) so that the caller receives exactly the information that you choose in the event of failure. This most often applies to library-like code. Use Anyhow if you don't care what error type your functions return, you just want it to be easy. This is common in application-like code.


> `tags` [[error]] [[cause]] [[parse_error]] [[box]] [[dyn]]