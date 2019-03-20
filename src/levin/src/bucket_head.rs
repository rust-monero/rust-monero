use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::LevinError;

pub const LEVIN_SIGNATURE: u64 = 0x0101010101012101;
pub const LEVIN_DEFAULT_TIMEOUT_PRECONFIGURED: usize = 0;
pub const LEVIN_DEFAULT_MAX_PACKET_SIZE: u64 = 100000000;

pub const LEVIN_PACKET_REQUEST: u32 = 0x00000001;
pub const LEVIN_PACKET_RESPONSE: u32 = 0x00000002;

pub const LEVIN_PROTOCOL_VER_0: u32 = 0;
pub const LEVIN_PROTOCOL_VER_1: u32 = 1;

pub const LEVIN_BUCKET_HEAD_LENGTH: usize = 33;

#[derive(Debug)]
pub struct BucketHead {
    pub signature: u64,
    pub cb: u64,
    pub have_to_return_data: bool,
    pub command: u32,
    pub return_code: i32,
    pub flags: u32,
    pub protocol_version: u32,
}

impl BucketHead {
    pub fn read<T: Buf>(buf: &mut T) -> Result<BucketHead, LevinError> {
        let bucket_head = BucketHead {
            signature: buf.get_u64_le(),
            cb: buf.get_u64_le(),
            have_to_return_data: buf.get_u8() != 0,
            command: buf.get_u32_le(),
            return_code: buf.get_i32_le(),
            flags: buf.get_u32_le(),
            protocol_version: buf.get_u32_le(),
        };
        println!("{:?}", &bucket_head);
        if bucket_head.signature != LEVIN_SIGNATURE {
            return Err(LevinError::SignatureInvalid);
        }

        if bucket_head.protocol_version != LEVIN_PROTOCOL_VER_1 {
            return Err(LevinError::InvalidProtocolVersion);
        }

        if bucket_head.cb > LEVIN_DEFAULT_MAX_PACKET_SIZE {
            return Err(LevinError::PackTooBig(bucket_head.cb));
        }

        if bucket_head.return_code < 0 {
            return Err(LevinError::ReturnCodeError(bucket_head.return_code));
        }
        Ok(bucket_head)
    }

    pub fn write(&self, buf: &mut BytesMut) {
        buf.reserve(LEVIN_BUCKET_HEAD_LENGTH);
        buf.put_u64_le(self.signature);
        buf.put_u64_le(self.cb);
        if self.have_to_return_data {
            buf.put_u8(1);
        } else {
            buf.put_u8(0);
        }
        buf.put_u32_le(self.command);
        buf.put_i32_le(self.return_code);
        buf.put_u32_le(self.flags);
        buf.put_u32_le(self.protocol_version);
    }

    pub fn is_request(&self) -> bool {
        self.flags & LEVIN_PACKET_REQUEST == LEVIN_PACKET_REQUEST
    }
}
