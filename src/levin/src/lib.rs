pub mod section;
pub mod raw_size;


pub enum LevinError {
    OutOfBound(usize),
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
