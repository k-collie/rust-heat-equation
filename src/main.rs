use std::fs::File;

use ndarray::NdProducer;
use ndarray::Zip;
use ndarray::prelude::*;
use ndarray::{self, IntoNdProducer};
use ndarray::{Slice, s};
use ndarray_linalg::Solve;
use ndarray_npy::create_new_npy;

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

fn main() {
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
    let mut u_left_slice = u.slice_mut(s![.., 0]);
    u_left_slice.iter_mut().for_each(|u| *u = U_LEFT);
    let mut u_right_slice = u.slice_mut(s![.., -1]);
    u_right_slice.iter_mut().for_each(|u| *u = U_RIGHT);

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
    for n in 0..N_T {
        let mut b = u.slice(s![n, 1..-1]).to_owned();

        // Add boundary condition contributions
        *b.first_mut().unwrap() += r * U_LEFT;
        *b.last_mut().unwrap() += r * U_RIGHT;

        // Solve linear system
        // u[n + 1, 1:-1] = np.linalg.solve(A, b)
        let sol = btcs_matrix.solve_into(b).unwrap();
        u.slice_mut(s![n + 1, 1..-1]).assign(&sol);
    }

    let mut npz_writer = ndarray_npy::NpzWriter::new(File::create("solution.npz").unwrap());
    npz_writer.add_array("x", &x).unwrap();
    npz_writer.add_array("t", &t).unwrap();
    npz_writer.add_array("u", &u).unwrap();
    npz_writer.finish().unwrap();
}
