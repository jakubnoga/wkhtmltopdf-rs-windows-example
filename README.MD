# Problem
https://github.com/anowell/wkhtmltopdf-rs#readme says
> This library using the libs (shared objects) and includes (headers) for PDF generation instead of the wkhtmltopdf executable.

# Solution 
1. Install wkhtmltopdf as https://github.com/anowell/wkhtmltopdf-rs#readme
    - Optionaly copy over `C:/Program Files/wkhtmltopdf/lib` (or your exact `wkhtmltopdf` location) to `/lib`
2. Use Rust's [Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) to point linker to wkhtmltox.lib
```rust
// build.rs
fn main() {
  // change "lib" to exact directory if you skipped optional part of point 1.
    println!(r"cargo:rustc-link-search=lib");
}
```
3. Copy over `C:/Program Files/wkhtmltopdf/bin/wkhtmltopdf.dll` to location discoverable by your Rust executable (ie. `C:/Windows/System32` or package-root)

# Run
This repo should be runnable **WITHOUT** applying `Solution` above with:
```powershell
cargo run # :)
```

# Contributors
Solution fully inspired by [OP and commenters in this issue](https://github.com/anowell/wkhtmltopdf-rs/issues/6)