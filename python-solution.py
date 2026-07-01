import numpy as np

# -----------------------------
# Problem parameters
# -----------------------------
L = 1.0  # domain length
ALPHA = 1.0  # thermal diffusivity
T = 0.2  # final time

N_X = 50  # number of spatial intervals
N_T = 100  # number of time steps

U_LEFT = 0.0  # boundary condition at x = 0
U_RIGHT = 0.0  # boundary condition at x = L


# -----------------------------
# Grid
# -----------------------------
dx = L / N_X
dt = T / N_T

x = np.linspace(0, L, N_X + 1)
t = np.linspace(0, T, N_T + 1)

r = ALPHA * dt / dx**2


# -----------------------------
# Initial condition
# -----------------------------
u = np.zeros((N_T + 1, N_X + 1))

u[0, :] = np.sin(np.pi * x)

# Apply boundary conditions
u[:, 0] = U_LEFT
u[:, -1] = U_RIGHT


# -----------------------------
# Build BTCS matrix
# -----------------------------
# We solve only for the interior points:
# x_1, x_2, ..., x_{Nx-1}
N_interior = N_X - 1

btcs_matrix = np.zeros((N_interior, N_interior))

for i in range(N_interior):
    btcs_matrix[i, i] = 1 + 2 * r

    if i > 0:
        btcs_matrix[i, i - 1] = -r

    if i < N_interior - 1:
        btcs_matrix[i, i + 1] = -r


# -----------------------------
# Time stepping
# -----------------------------
for t_idx in range(N_T):
    b = u[t_idx, 1:-1].copy()

    # Add boundary condition contributions
    b[0] += r * U_LEFT
    b[-1] += r * U_RIGHT

    # Solve linear system
    u[t_idx + 1, 1:-1] = np.linalg.solve(btcs_matrix, b)


# -----------------------------
# Save solution
# -----------------------------
np.savez("solution.npz", x=x, t=t, u=u)
