# Hashtest

This Rust library may be used to test if a set of files has changed on disk. To do this, Hashtest maintains a file on disk which stores hashes for the supplied files and uses this cache to test for changes. 

# Design Notes

## On Testing
A lot of the engineering here is in service of testing. Rather than rely on what would be a strait forward concrete implementation, Hashit provides a number of traits which, along with Read and Write, allow it to implement a testing strategy using Strings, in addition to files. 