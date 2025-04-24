# Sighs

**Cute little rust library to determine file and directory size**

This crate silently ignores all errors, so use it wisely. Works well with
`size`. If `filesize()` is called on a directory, 0 is returned.

**Examples**
```rust
use sighs::filesize;
use size::Size;

let bytes = filesize("file");
println!("Size of 'file': {}", Size::from_bytes(bytes).format())
```

```rust
use sighs::dirsize;
use size::{Size, Base::Base10};

let bytes = dirsize("dir");
println!("Size of 'dir': {}", Size::from_bytes(bytes).format().with_base(Base10))
```

***Hint:** Look at the tests in `src/lib.rs` for more usage examples.*
