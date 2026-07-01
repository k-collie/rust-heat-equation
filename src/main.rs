use std::fs::File;

use ndarray::{prelude::*, s};
use ndarray_linalg::Solve;

// -----------------------------
// Problem parameters
// -----------------------------
const L: f64 = 1.0; // domain length
const ALPHA: f64 = 1.0; // thermal diffusivity
const T: f64 = 0.2; // final time

const N_X: usize = 50; // number of spatial intervals
const N_T: usize = 100; // number of time steps

const U_LEFT: f64 = 0.0; // boundary condition at x = 0
const U_RIGHT: f64 = 0.0; // boundary condition at x = L

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -----------------------------
    // Grid
    // -----------------------------
    let dx = L / (N_X as f64);
    let dt = T / (N_T as f64);

    let x = ndarray::Array1::linspace(0., L, N_X + 1);
    let t = ndarray::Array1::linspace(0., T, N_T + 1);

    let r = ALPHA * dt / dx.powi(2);

    // -----------------------------
    // Initial condition
    // -----------------------------
    let mut u = ndarray::Array2::zeros((N_T + 1, N_X + 1));

    let u_t0 = u.slice_mut(s![0, ..]);

    ndarray::Zip::from(u_t0)
        .and(&x)
        .for_each(|u_x, &x| *u_x = f64::sin(core::f64::consts::PI * x));

    // Apply boundary conditions
    u.slice_mut(s![.., 0]).fill(U_LEFT);
    u.slice_mut(s![.., -1]).fill(U_RIGHT);

    // -----------------------------
    // Build BTCS matrix
    // -----------------------------
    // We solve only for the interior points:
    // x_1, x_2, ..., x_{Nx-1}
    const N_INTERIOR: usize = N_X - 1;

    let mut btcs_matrix = ndarray::Array2::zeros((N_INTERIOR, N_INTERIOR));

    for i in 0..N_INTERIOR {
        btcs_matrix[[i, i]] = 1. + 2. * r;

        if i > 0 {
            btcs_matrix[[i, i - 1]] = -r;
        }

        if i < N_INTERIOR - 1 {
            btcs_matrix[[i, i + 1]] = -r;
        }
    }

    // -----------------------------
    // Time stepping
    // -----------------------------
    let mut b = Array1::<f64>::zeros(N_INTERIOR);
    for t_idx in 0..N_T {
        #[allow(clippy::reversed_empty_ranges)]
        b.assign(&u.slice(s![t_idx, 1..-1]));
        // Add boundary condition contributions
        b[0] += r * U_LEFT;
        b[N_INTERIOR - 1] += r * U_RIGHT;

        // Solve linear system
        // u[n + 1, 1:-1] = np.linalg.solve(A, b)
        let sol = btcs_matrix.solve(&b)?;
        #[allow(clippy::reversed_empty_ranges)]
        u.slice_mut(s![t_idx + 1, 1..-1]).assign(&sol);
    }

    // -----------------------------
    // Save solution
    // -----------------------------
    let mut npz_writer = ndarray_npy::NpzWriter::new(File::create("solution.npz")?);
    npz_writer.add_array("x", &x)?;
    npz_writer.add_array("t", &t)?;
    npz_writer.add_array("u", &u)?;
    npz_writer.finish()?;

    Ok(())
}
