pub mod bucket;
pub mod bucket_head;
pub mod raw_size;
pub mod section;

#[derive(Debug)]
pub enum LevinError {
    InvalidSerializeType(u8),
    OutOfBound(usize),
    ErrorArrayType(u8),
    SignatureInvalid,
    InvalidProtocolVersion,
    PackTooBig(u64),
    ReturnCodeError(i32),
}

#[macro_export]
macro_rules! ensure_eof {
    ($buf:expr, $needed:expr) => {
        if $buf.remaining() < $needed {
            return Err($crate::LevinError::OutOfBound($needed));
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
