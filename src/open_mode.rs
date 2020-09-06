#[derive(Debug, PartialEq, Eq)]
pub enum OpenMode {
    WriteAppend,
    WriteTruncate,
}
