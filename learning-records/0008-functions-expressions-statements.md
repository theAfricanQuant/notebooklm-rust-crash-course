# Functions, expressions, and statements practiced

The learner created `practice/day-08-functions` in Zed and built the program in small steps, running each version with `cargo fmt && cargo run`.

## Evidence

- Defined and called a no-argument function, then passed an `i32` parameter to another function.
- Implemented `add_numbers(left: i32, right: i32) -> i32`, which returned `23` for inputs `16` and `7`.
- Added a semicolon after `left + right` and observed compiler error E0308: the function promised `i32` but its body produced `()`.
- Removed the semicolon and confirmed the sum returned correctly.
- Implemented `days_until_goal`, which returned `7` when given 23 days and a goal of 30, and returned `0` when the goal was already reached.
- Described the distinction in Python terms: a statement can bind a variable without producing a value, while an expression evaluates to a value.

## Demonstrated understanding

The learner connected Rust's final-expression return to Python's explicit `return`: `left + right` evaluates to an integer, and without a trailing semicolon it becomes the function's return value. The `let result = left + right;` line is a statement that binds that value; it does not itself produce a value for the surrounding function.
