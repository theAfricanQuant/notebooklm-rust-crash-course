# Arithmetic and type casting practiced

The learner created `practice/day-06-arithmetic-casting`, entered whole-number weekly study totals in Zed’s integrated terminal, and ran `cargo fmt && cargo run` successfully.

## Evidence

With an input of `16`, the program printed a whole-number daily average of `2`, a remainder of `2`, and a precise average of `2.2857142857142856`. With an input of `95`, it printed `13`, `4`, and `13.571428571428571` respectively.

The learner identified the reason for the difference as integer versus floating-point calculation, using the corresponding Python intuition. In Rust terms, `i64 / i64` produces an integer result and drops the fractional part; `f64 / f64` preserves it. Rust requires the operands of an arithmetic operation to have compatible types, so the program explicitly casts both values to `f64` for the precise average.

## Demonstrated understanding

The learner can distinguish a whole-number result plus remainder from a decimal result, and can explain why `95 / 7` differs from `95.0 / 7.0`. The next retrieval should ask when an explicit `as` cast is appropriate and why narrowing a number type can lose information.
