import matplotlib.pyplot as plt
import numpy as np

npz = np.load("solution.npz")
x = npz["x"]
t = npz["t"]
u = npz["u"]

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
