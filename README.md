# Rust Vector Reference Invalidation

This repository demonstrates a common error in Rust involving references to vector elements.  When you obtain a reference to an element in a vector using `get()`, that reference's validity is tied to the vector's capacity and memory layout. Modifying the vector (adding, removing, or resizing) after obtaining this reference can invalidate it, leading to undefined behavior or panics at runtime. 

The `bug.rs` file showcases this issue. The `bugSolution.rs` file offers a way to mitigate this by making a copy if necessary.

This is an important concept to understand when working with vectors and references in Rust.