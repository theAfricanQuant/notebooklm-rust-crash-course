# Stack and heap memory practiced

The learner created `practice/day-09-memory` in Zed, ran a program with an `i32` and a growable `String`, then changed the text and reran it.

## Evidence

- The program printed `Score: 95`, `Language: Rust`, and a string length of 4 bytes.
- After changing the string to `Rust is fun!`, the learner observed a length of 12 bytes.
- The learner explained that the `String` handle—including its pointer, length, and capacity—is on the stack, while the text characters are on the heap. They also identified the fixed-size integer `95` as stored directly on the stack in the lesson's simple model.

## Demonstrated understanding

The learner can distinguish a fixed-size integer from growable string data and describe the simple stack/heap model used in this lesson. The next topic, ownership, can build on this memory model.
