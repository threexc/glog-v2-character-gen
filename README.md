# GLOG v2 Character Generator

This is a simple character generator for the [Many Rats on
Sticks](https://drive.google.com/file/d/1wOAkBOCUSjnthMEnIsPVT1LSOCQzd88j/view)
game system, also known as GLOG v2.

Run the `stats` binary with e.g.:

3d6: `cargo run --bin stats -- --dice 3 --sides 6`

4d6 drop lowest: `cargo run --bin stats -- --dice 4 --sides 6 --lowest 1`

Defaults to 3d6.

Run the CLI with:

```
cargo run --bin cli
```

or run the web version:

```
cargo run --bin web --features web
```
