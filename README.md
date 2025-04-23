# rust-fe-calculator (in development)
A simple 2D Finite Element (FE) application written in Rust.

This application parses model inputs from `.txt` files, making it easy to modify and iterate on simulations.

**Status**: This project is currently under active development.

## How to Run
- Modify the input txt files in the `inputs/` directory (keypoints, connections, pointloads, boundary-conditions).
- Set material parameters (e.g., `E`, `A`) directly in `main.rs`.
- Build and run the application using:
   ```bash
   cargo run
   ```

## Method
The application follows these steps:

- Parses inputs and loads them into Rust structs.
- Builds the global stiffness matrix **[K]** using keypoints and connection inputs.
- Applies boundary conditions based on boundary definitions.
- Constructs the global force vector **[F]** based on pointload inputs.
- Solves for displacements using:

  ```
  [u] = [K]⁻¹ · [F]
  ```

- Calculates internal nodal forces using:

  ```
  [F] = [K] · [u]
  ```

## Elements
- Bar elements (axial stiffness only).

## Tech Stack
- **Rust**.
- **nalgebra** – linear algebra for matrix/vector operations.
- Plain text input format – simple and flexible `.txt` files.
- Custom-built FE core logic from scratch.

## Future Implementation Ideas
- Add support for beam elements (bending).
- Create a visualization module for geometry and force + displacement overview.
- Add spring stiffness boundary conditions.
- Add test cases.
