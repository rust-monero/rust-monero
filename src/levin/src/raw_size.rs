extern crate bytes;

use bytes::{Buf, BufMut, BytesMut};

use super::*;

const PORTABLE_RAW_SIZE_MARK_MASK: u8 = 0x03;
const PORTABLE_RAW_SIZE_MARK_BYTE: u8 = 0;
const PORTABLE_RAW_SIZE_MARK_WORD: u8 = 1;
const PORTABLE_RAW_SIZE_MARK_DWORD: u8 = 2;
const PORTABLE_RAW_SIZE_MARK_INT64: u8 = 3;

pub fn read(buf: &mut Buf) -> Result<usize, LevinError> {
    ensure_eof!(buf, 1);
    let mark = buf.bytes()[0] & PORTABLE_RAW_SIZE_MARK_MASK;
    match mark {
        PORTABLE_RAW_SIZE_MARK_BYTE => Ok((buf.get_u8() >> 2) as usize),
        PORTABLE_RAW_SIZE_MARK_WORD => {
            ensure_eof!(buf, 2);
            Ok((buf.get_u16_le() >> 2) as usize)
        }
        PORTABLE_RAW_SIZE_MARK_DWORD => {
            ensure_eof!(buf, 4);
            Ok((buf.get_u32_le() >> 2) as usize)
        }
        PORTABLE_RAW_SIZE_MARK_INT64 => {
            ensure_eof!(buf, 8);
            Ok((buf.get_u64_le() >> 2) as usize)
        }
        _ => unreachable!(),
    }
}

pub fn write(buf: &mut BytesMut, val: usize) {
    if val <= 63 {
        buf.reserve(1);
        buf.put_u8(((val as u8) << 2) | PORTABLE_RAW_SIZE_MARK_BYTE);
    } else if val <= 16383 {
        buf.reserve(2);
        buf.put_u16_le(((val as u16) << 2) | PORTABLE_RAW_SIZE_MARK_WORD as u16);
    } else if val <= 1073741823 {
        buf.reserve(4);
        buf.put_u32_le(((val as u32) << 2) | PORTABLE_RAW_SIZE_MARK_DWORD as u32);
    } else if val as u64 <= 4611686018427387903 {
        buf.reserve(8);
        buf.put_u64_le(((val as u64) << 2) | PORTABLE_RAW_SIZE_MARK_INT64 as u64);
    } else {
        /// XXX: Hope some day monero never uses a value too large.
        panic!("too large");
    }
}
