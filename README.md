# Hashtest

This Rust library may be used to test if a set of files has changed on disk. To do this, Hashtest maintains a file on disk which stores hashes for the supplied files and uses this cache to test for changes. 

# Design Notes

## On Testing
A lot of the engineering here is in service of testing. Rather than rely on what would be a strait forward concrete implementation, Hashit provides a number of traits which, along with Read and Write, allow it to implement a testing strategy using ```String```s, in addition to ```File```s. To accomplish this, I rely on lazy static initialization of a HashMap of type ```HashMap<String, Vec<u8>>```; the String represents a path, and the Vec<u8> stores a hash. Additionally, I provide a custom type ResourceReaderWriter, which implements Read and Write, and which is used in place of ```io::File``` in the test suite. It does impose one additional requirement. Since the HashMap is global singleton, I am forced to run all tests serially, using the excellent ```serial_test``` library. Also, I am forced to wrap the HashMap in a Mutex. ( I probably could get rid of the serial_test by taking a lock on the Mutex on the first line of each test, but that would have the same effect since only one thread would be able to make progress at a time).

Overall, to solve the concrete problem took maybe an hour (probably less). To come up with abstractions that allowed for testing took much longer (5-10x)

