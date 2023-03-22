# 🗂️ stateman

Runtime managed resource borrowing.

This library provides a map that can store one of any type, as well as
mutable borrows to each type at the same time.

**Note:** This implementation is forked from [`resman`](https://github.com/azriel91/resman), with the
following differences:

* `Resource` is not required to be `Sync + 'static`.
* Uses [`better_any`] instead of [`downcast-rs`] for downcasting types with lifetime.
* `"fn_res"`, `"fn_res_mut"`, `"fn_meta"` features were removed.

## Usage

Add the following to `Cargo.toml`

```toml
stateman = { git = "https://github.com/Saethox/stateman" }

# or
stateman = { git = "https://github.com/Saethox/stateman", features = ["debug"] }
```

In code:

```rust
use better_any::Tid;

use stateman::Resources;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Tid)]
struct A(u32);

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Tid)]
struct B(u32);

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Tid)]
struct C<'a>(&'a A);

fn main() {
    // `A` must live longer than `resources`.
    let owned_a = A(3);

    let mut resources = Resources::default();

    resources.insert(A(1));
    resources.insert(B(2));
    
    // Note how `C` has a lifetime.
    resources.insert(C(&owned_a));

    // We can validly have two mutable borrows from the `Resources` map!
    let mut a = resources.borrow_mut::<A>();
    let mut b = resources.borrow_mut::<B>();
    a.0 = 2;
    b.0 = 3;

    // We need to explicitly drop the A and B borrows, because they are runtime
    // managed borrows, and rustc doesn't know to drop them before the immutable
    // borrows after this.
    drop(a);
    drop(b);

    // Multiple immutable borrows to the same resource are valid.
    let a_0 = resources.borrow::<A>();
    let _a_1 = resources.borrow::<A>();
    let b = resources.borrow::<B>();
    let c = resources.borrow::<C>();

    println!("A1: {}", a_0.0);
    println!("B: {}", b.0);
    println!("C: {}", c.0.0);

    // Trying to mutably borrow a resource that is already borrowed (immutably
    // or mutably) returns `Err`.
    let a_try_borrow_mut = resources.try_borrow_mut::<A>();
    let exists = if a_try_borrow_mut.is_ok() {
        "Ok(..)"
    } else {
        "Err"
    };
    println!("a_try_borrow_mut: {}", exists); // prints "Err"

    println!("{resources:?}");
}
```

### Features

#### `"debug"`:

The `Debug` implementation for `Resources` will use the `Debug`
implementation for the values when printed. This requires that all
`Resources` to also implement `Debug`.

Example:

```rust
use stateman::Resources;

let mut resources = Resources::default();
resources.insert(1u32);
println!("{:?}", resources);

// Without `"debug"` feature:
// {u32: ".."}

// With `"debug"` feature:
// {u32: 1}
```

## See Also

* [`resman`]: Upstream repository of this fork.
* [`anymap`]: Map of any type, without multiple mutable borrows.
* [`rt_map`]: Runtime managed mutable borrowing from a map.
* [`shred`]: Contains `Resources` type, plus a task dispatcher.

[`anymap`]: https://github.com/chris-morgan/anymap
[`resman`]: https://github.com/azriel91/resman
[`better_any`]: https://github.com/luleyleo/better_any
[`downcast-rs`]: https://github.com/marcianx/downcast-rs
[`mopa`]: https://github.com/chris-morgan/mopa
[`rt_map`]: https://github.com/azriel91/rt_map
[`shred`]: https://github.com/amethyst/shred

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT