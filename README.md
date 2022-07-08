### Safe implementation of the `Iterator` trait for `Generator`

## Usage

```rust
#![feature(generators)]

use yield_iter::generator;

fn main() {
    let x = 10;
    let iter = generator! {
        let r = &x;
        for i in 0..5u32 {
            yield i * *r
        }
    };
}
```