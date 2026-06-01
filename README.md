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

## Benchmarks

The benchmark suite includes small test graphs and a few large graphs from the USA-road-d dataset. You can run all of these by using the `runner.py` script. The results are summarized below:

| Solver | test1.gr | test2.gr | USA-road-d.BAY.gr | USA-road-d.NY.gr |
|--------|----------|----------|-------------------|------------------|
| `ch` | 0.01s | 0.01s | 0.58s | 0.43s |
| `indexed_heap` | 0.02s | 0.02s | 3.24s | 1.55s |
| `dijkstra` | 0.03s | 0.03s | 3.49s | 2.73s |
| `bellman_ford` | 0.04s | 0.02s | Timeout (>60s) | Timeout (>60s) |
