pub mod section;
pub mod raw_size;

#[derive(Debug)]
pub enum LevinError {
    InvalidSerializeType(u8),
    OutOfBound(usize),
    ErrorArrayType(u8)
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
