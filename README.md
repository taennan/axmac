
# Axmac

Readable indexing for 1-4 dimensional data structures.

### The Problem

Have you ever tried to index collections like so:

```rust
let point3d = [0.32, 1.2, 10.7];
let first = point3d[0];
let next  = point3d[1];
// ...
```

Trust me, it gets annoying quickly.

The `axmac` crate was designed to be a solution to your indexing problems.

## Basic Usage

This crate provides the macros `ax!`, `axs!` and `axr!` which transform 
the identifiers _x_, _y_, _z_ and _w_ into `usize` values.

### ax! (_axis_)

Converts a single identifier into a `usize` value

```rust
// Note:
assert_eq!(ax!(x), 0usize);
assert_eq!(ax!(y), 1);
assert_eq!(ax!(z), 2);
assert_eq!(ax!(w), 3);

let arr = ["a", "b", "c", "d"];
assert_eq!(arr[ax!(y)], "b");
assert_eq!(arr[ax!(z)], "c");
```

### axr! (_axis range_)

Converts a range of identifiers and/or expressions into a range of `usize`'s

```rust
// Here are just a few of the variations
let range1 = axr!(x..=z);
let range2 = axr!(z..4);
let range3 = axr!((1)..z);

assert_eq!(range1, 0..=3);
assert_eq!(range2, 2..4);
assert_eq!(range3, 1..2);

let arr = ["a", "b", "c", "d"];
assert_eq!(arr[range1], ["a", "b", "c"]);
assert_eq!(arr[range2], ["c", "d"]);
assert_eq!(arr[range3], ["b"]);
```

### axs! (_axes_)

Converts an array of identifiers into an array of `usize`'s

```rust
let array = axs![x, y, x, w];
assert_eq!(array, [0, 1, 0, 3]);

let array = axs![z; 4];
assert_eq!(array, [2, 2, 2, 2]);
```

## Contributing

Any suggestions for the codebase, documentation, README (or anything) are more than welcome!

If there are any problems or queries, please submit an issue on our Github page.

## License

This crate is available under the `MIT` and/or `Apache2.0` licenses.