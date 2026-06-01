# shortest-path-solvers
Playground for trying out different shortest path solvers

## Rust CH Solver

A high-performance Contraction Hierarchies solver implemented in Rust is included to provide microsecond-level queries on massive road networks.

### Building and Executing
The central `runner.py` script executes the solver seamlessly. To run it:
```bash
./runner.py ch
```
The `solvers/ch/solve` wrapper script will automatically invoke Cargo to compile the Rust binary in release mode (if it has not been built yet) and execute it. 

### Manual Compilation
If you wish to build the solver manually:
```bash
cd solvers/ch
cargo build --release
```
The standalone executable will be available at `solvers/ch/target/release/ch`.
