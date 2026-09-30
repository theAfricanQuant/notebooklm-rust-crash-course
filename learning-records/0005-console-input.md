# Console input practiced

The learner created `practice/day-05-console-input`, used Zed’s integrated terminal, and ran `cargo fmt && cargo run` successfully.

## Evidence

The program asked, `What name should I use?`; the learner entered `SisengAI`; and the program printed `Hello, SisengAI`.

## Demonstrated understanding

The learner completed the console-input lab. The program starts with an empty `String`, then `read_line(&mut name)` updates that same string with terminal input. The next retrieval should ask why the binding needs `mut` and what temporary permission `&mut name` gives the input function.
