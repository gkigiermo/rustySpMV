# SpMV in rust

The sparse matrix-vector multiplication is ubiquitous in scientific applications. In this repo, we work with matrices arising from computational fluid dynamics simulations.

For simplicity, the input matrices are stored in CSR format.

To compile and run just execute:
```
cargo run -- --matrix-file data/100K.csr --repetitions 10
```
It demands two parameters:

`matrix-file`: which is the file that contains the matrix in CSR format.

`repetitions`: how many times the test is repeated to obtain a representative average.

The output is the elapsed time in milliseconds.
