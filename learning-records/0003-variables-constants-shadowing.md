# Variables, constants, and shadowing practiced

The learner created `practice/day-03-variables` with Cargo, replaced the generated program, and ran `cargo fmt && cargo run` successfully from Zed.

## Evidence

The program printed the expected scope and value sequence: outer `x` starts at 4; shadowed `x` is 5; inner-scope `x` is 2; outer `x` remains 5; a new shadowed `x` can be text; mutable `deliveries` becomes 4; and `SECONDS_PER_MINUTE` prints 60.

The learner then removed `mut` from `let mut deliveries = 3;`. Before running Cargo, Zed reported Rust error E0384: the binding must be mutable because the next line reassigns `deliveries`. Screenshot: `assets/day-03-immutable-warning.png`. Running `cargo fmt && cargo run` produced the same E0384 compiler error and suggested `let mut deliveries = 3;`: `assets/day-03-cargo-immutable-error.png`.

## Demonstrated understanding

The learner noticed that `let x = x + 1;` and `x = x + 1;` look similar but asks why one is permitted. The next retrieval should confirm the distinction: the former creates a new binding by shadowing; the latter reassigns the existing binding and therefore requires `mut`. A Day 3 clarification now explicitly teaches scope: the inner `x = 2` exists only inside its braces and does not alter the outer `x = 5`.
