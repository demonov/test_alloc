# test_alloc

When you run this program, it will output the initial and final memory usage, as well as the difference between them:

```sh
cargo run --release sync 100
```
```sh
cargo run --release async 100
```

This information will help you verify that there is no memory leak in both synchronous and asynchronous modes. If there is no memory leak, the difference in memory usage should be close to zero after the program has completed all the iterations.