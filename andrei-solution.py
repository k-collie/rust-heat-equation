import matplotlib.pyplot as plt
import numpy as np

# -----------------------------
# Problem parameters
# -----------------------------
L = 1.0  # domain length
alpha = 1.0  # thermal diffusivity
T = 0.2  # final time

Nx = 50  # number of spatial intervals
Nt = 100  # number of time steps

u_left = 0.0  # boundary condition at x = 0
u_right = 0.0  # boundary condition at x = L


# -----------------------------
# Grid
# -----------------------------
dx = L / Nx
dt = T / Nt

x = np.linspace(0, L, Nx + 1)
t = np.linspace(0, T, Nt + 1)

r = alpha * dt / dx**2


# -----------------------------
# Initial condition
# -----------------------------
u = np.zeros((Nt + 1, Nx + 1))

u[0, :] = np.sin(np.pi * x)

# Apply boundary conditions
u[:, 0] = u_left
u[:, -1] = u_right


# -----------------------------
# Build BTCS matrix
# -----------------------------
# We solve only for the interior points:
# x_1, x_2, ..., x_{Nx-1}
N_interior = Nx - 1

A = np.zeros((N_interior, N_interior))

for i in range(N_interior):
    A[i, i] = 1 + 2 * r

    if i > 0:
        A[i, i - 1] = -r

    if i < N_interior - 1:
        A[i, i + 1] = -r


# -----------------------------
# Time stepping
# -----------------------------
for n in range(Nt):
    b = u[n, 1:-1].copy()

    # Add boundary condition contributions
    b[0] += r * u_left
    b[-1] += r * u_right

    # Solve linear system
    u[n + 1, 1:-1] = np.linalg.solve(A, b)


# -----------------------------
# Plot solution
# -----------------------------
X, T_grid = np.meshgrid(x, t)

fig = plt.figure(figsize=(9, 6))
ax = fig.add_subplot(111, projection="3d")

ax.plot_surface(X, T_grid, u, cmap="viridis")

ax.set_xlabel("x")
ax.set_ylabel("t")
ax.set_zlabel("u(x,t)")
ax.set_title("1D Heat Equation solved by BTCS")

plt.tight_layout()
plt.show()
