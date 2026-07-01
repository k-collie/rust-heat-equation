# "Hello world" to Heat Equation

The aim of this course is to go from "hello world" in Rust
to solving the 1D heat equation implicitly with
a BTCS finite difference scheme.

We aim to write a Rust version of the code in `python-solution.py`.

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
let mut zeros = ndarray::Array2::zeros((10, 100));
```

By default variables are immutable but we used
`let mut` here to create a mutable variable
so we can later write to this array.

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
- faer-rs sparse solution
    - https://faer.veganb.tw/docs/sparse-linalg/
- eliminate allocations in loop?
- lint code with clippy
    - if you have a slice like `s![1..-1]` clippy
      will think that it is empty. You can disable this warning
      with `#[allow(clippy::reversed_empty_ranges)]` above
      lines that slice like this.
- add some command line arguments
    - set boundary conditions?
    - https://crates.io/crates/clap
- implement Thomas algorithm for tri-diagonal matrices
