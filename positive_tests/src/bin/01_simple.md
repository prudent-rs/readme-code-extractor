```rust
#[::no_panic::no_panic]
fn divide() {
     core::hint::black_box( 1/1 );
}

#[::no_panic::no_panic]
fn slice_access() {
    const A: [bool; 2] = [true, false];
    let s = &A[..];
    core::hint::black_box( s[0] );
}

divide();
slice_access();
```
