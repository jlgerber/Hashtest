# Hashtest

This Rust library may be used to test if a set of files has changed on disk. To do this, Hashtest maintains a file on disk which stores hashes for the supplied files and uses this cache to test for changes. 

# Design Notes

## On Testing
A lot of the engineering here is in service of testing. Rather than rely on what would be a strait forward concrete implementation, Hashit provides a number of traits which, along with Read and Write, allow it to implement a testing strategy using Strings, in addition to files. 

### Todo
On the testing front, I am not quite happy with how simplified the string version is. I want to close the gap between the file and string version by relying on a lazy static HashMap stored behind a global reference.
[see example](https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html)
This would be
```rust
type ResourceHash = Vec<u8>;
type ResourceHashMap = HashMap<String, ResourceHash>;
```
This should allow me to store a dictionary of string keys and hash values