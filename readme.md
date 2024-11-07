# Vector Operations

This is a collection of vector operations for Rust. 

## Usage

To use this crate, add the following to your `Cargo.toml` file:

```toml
[dependencies]
vector_operations = "0.1.0"
```

Then, add this to your crate root:

```rust
use vector_operations::*;
```

## Examples

### Addition

```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];

let c = add(a, b);

assert_eq!(c, vec![5, 7, 9]);
```

### Subtraction

```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];

let c = sub(a, b);

assert_eq!(c, vec![-3, -3, -3]);
```

### Scaling

```rust
let a = vec![1, 2, 3];
let b = 2;

let c = scale(a, b);

assert_eq!(c, vec![2, 4, 6]);
```

### Matrix-Vector Multiplication

```rust
let a = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
];
let b = vec![1, 2, 3];

let c = mat_vec_mul(a, b);

assert_eq!(c, vec![30, 36, 42]);
```