# "Hello world" to Heat Equation

The aim of this course is to go from "hello world" in Rust
to solving the 1D heat equation implicitly with
a BTCS finite difference scheme.

We aim to write a Rust version of the code in `python-solution.py`.
You can have `python-solution.py` open alongside
your Rust solution as you work on it.

Running

```bash
python python-solution.py
```

outputs a file `solution.npz` which contains `x`, `t` and `u` array.
We can plot these with:

```bash
python plot.py
```

# Course

## Resources

- [rust book](https://doc.rust-lang.org/book/)
- [ndarray docs](https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html)
- [ndarray_linalg docs](https://docs.rs/ndarray-linalg/latest/ndarray_linalg/index.html)
- [NpzWriter docs](https://docs.rs/ndarray-npy/latest/ndarray_npy/struct.NpzWriter.html)

## Installing Rust

To install Rust go to [rustup.rs](https://rustup.rs/).
Copy the command and paste it into your command line.
Restart your shell or
run the command to source the correct environment.

## Starting a new Rust project

Create a binary package called `rust-heat-equation`:

```bash
cargo new --bin rust-heat-equation
cd rust-heat-equation
```

Build and run the "Hello world" default code:

```bash
cargo run
```

If we want to run an optimized build we use `--release`:

```bash
cargo run --release
```

By default Rust compiles a more portable binary.
To use the latest instructions available for your CPU
you can set `RUSTFLAGS`:

```bash
export RUSTFLAGS="-C target-cpu=native"
```

Lets looks at what's in `src/main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

- `main` is the usual entry point function
- `println!` is a macro (macros end in `!`)

## Adding dependencies

We use some dependencies to handle arrays, do linear algebra and save
arrays in numpy format.

```bash
cargo add ndarray --no-default-features --features std
cargo add ndarray-linalg --no-default-features --features openblas-static
cargo add ndarray-npy --no-default-features --features compressed_npz
cargo build
```

Rust can have features to conditionally compile different parts of code.
We disable the default features and manually specify features we want:
- `std` allows access to the standard library, for some targets (think
  microcontrollers) the standard library isn't available.
- `openblas-static` tells `ndarray-linalg` to build `openblas` as part of the
  project (rather than finding a system version) and link to it statically.
- `compressed_npz` allows `ndarray-npy` to write compressed `.npz` files

We ran `cargo build` early because `openblas` takes a while to build.

We can have a look at what these commands did to our `Cargo.toml` and
`Cargo.lock` files:

```bash
cat Cargo.toml
less Cargo.lock
```

## Importing things

We need to import some things that we'll use later,
we do this with `use`:

```rust
use std::fs::File;

use ndarray::{prelude::*, s};
use ndarray_linalg::Solve;
```

`ndarray_linalg::Solve` is a trait and must be in scope
in order for us to use the methods it provides.

## Constants

Compile time constants in Rust can be defined like this:

```rust
const MY_CONST: u32 = 5;
```

Constants are normally `UPPER_SNAKE_CASE`.

We must specify the type for constants,
`u32` is a 32-bit unsigned integer.
Other primitive types we'll use are:
- `f64` - 64-bit floating point number
- `usize` - size of pointer, useful for array lengths and indexing
  (like `std::size_t` in C++)
- `bool` - we use these but you probably won't need to name it.
  Can be `true` or `false`

## Assigning variables

To assign variables we use `let`, e.g.:

```rust
let dx = L / (N_X as f64);
```

Here `L` is a float and `N_X` is `usize`.
We can convert `N_X` to a float before dividing with `as`.

To assign arrays with linearly spaced elements we can use
[`ndarray::ArrayBase::linspace`](
https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#method.linspace
):

```rust
let floats_between_0_and_10 = ndarray::Array1::linspace(0., 10., 100);
```

For an array full of zeros we can use
[`ndarray::ArrayBase::zeros`](
https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html#method.zeros
).

```rust
let mut zeros: ndarray::Array2<f64> = ndarray::Array2::zeros((5, 10));
```

By default variables are immutable but we used
`let mut` here to create a mutable variable
so we can later write to this array.

## Debug printing

Rust has a macro `dbg!` which lets us print anything that implements the
`Debug` trait (most things). This is very useful for print debugging.

```rust
dbg!(&zeros);
```

## Maths with f64

We can calculate a square with an integer power like this:

```rust
f64::powi(8., 2)
```

We can do sine like this:

```rust
f64::sin(3. * core::f64::consts::PI)
```

## Indexing

Rust is zero indexed.

We can index like this:

```rust
let num = zeros[[1, 3]];
```

We can assign too:

```rust
zeros[[1, 3]] = 5.;
```

## Slicing

We can slice:

```rust
let small_slice = zeros.slice(s![.., 0..1]);
```

We can use `-1` to wrap round and mean the last element
(just like python):

```rust
let big_slice = zeros.slice(s![.., 1..-1]);
```

We can get mutable slices:

```rust
let mut big_mutable_slice = zeros.slice_mut(s![.., 1..-1]);
```

## Assigning to arrays

If we have a mutable array or slice we can fill it with the `fill`:

```rust
big_mutable_slice.fill(3.);
```

If we have arrays or slices of the same size
we can assign from one to the other:

```rust
let mut arr1 = ndarray::Array1::zeros(5);
let arr2 = ndarray::Array1::ones(5);
arr1.assign(&arr2);
```

Assign takes an immutable reference so we use `&`.

## Zipping and iterators

We can zip two arrays or slice together and iterate over them:

```rust
let linspaced = ndarray::Array1::linspace(0., 1., 10);
let mut squared = ndarray::Array1::zeros(10);
ndarray::Zip::from(&linspaced)
    .and(&mut squared)
    .for_each(|x, y| y = f64::powi(x, 2));
```

## for loops

We can loop between `0` up to `N_INTERIOR` exclusive:

```rust
for i in 0..N_INTERIOR {
    println!("i = {i}");
}
```

We can loop between `0` up to `N_INTERIOR` or inclusive:

```rust
for i in 0..=N_INTERIOR {
    println!("i = {i}");
}
```

## Returning Result from main

Rust lets us return errors from `main`.
That way if we do something fallible (like creating a file),
we can use the `?` operator to make an early return on error:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("my-file.txt")?;
    Ok(())
}
```

The `Box<dyn ...>` allows us to return
anything that implements the `std::error::Error` trait.

We also made sure to return a `Result` at the end of `main`.
If we got to the end then everything went fine, so we return `Ok(())`,
which is the `Ok` variant of the `Result` enum containing the empty type `()`.

## Solving

We can solve matrix equations `Ax = b` like this:

```rust
let x = A.solve(&b)?;
```

## Writing files

Once you have arrays to write,
you can write them to a `.npz` file like this:

```rust
let file = File::create("solution.npz")?;
let mut npz_writer = ndarray_npy::NpzWriter::new(file);
npz_writer.add_array("x", &x)?;
npz_writer.add_array("t", &t)?;
npz_writer.add_array("u", &u)?;
npz_writer.finish()?;
```

# Extension ideas

- factor out functionality and add unit tests, e.g.
    - `setup_ics_bcs_u`
    - `assemble_btcs_matrix`
    - `solve_timestep`
    - `save_solution`
    - see [the book on tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- LU factorize matrix out of loop
- Use a tridiagonal matrix type:
    - https://docs.rs/ndarray-linalg/latest/ndarray_linalg/tridiagonal/index.html
- eliminate allocations in loop?
    - check out `ndarray-linalg` `*_inplace` methods
- lint code with clippy
    - if you have a slice like `s![1..-1]` clippy
      will think that it is empty. You can disable this warning
      with `#[allow(clippy::reversed_empty_ranges)]` above
      lines that slice like this.
- add some command line arguments
    - set boundary conditions?
    - https://crates.io/crates/clap
- faer-rs sparse solution
    - I haven't tried this yet so might not be much help
    - https://faer.veganb.tw/docs/sparse-linalg/
- use `thiserror` replace `Box<dyn Error>`
    - this is an ergonomic way to make error enums,
      which enumerate the possible ways a function can go wrong
      and helps code which handles these errors
    - https://crates.io/crates/thiserror
