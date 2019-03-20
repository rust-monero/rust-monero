use bytes::{BufMut, BytesMut};

use crate::bucket_head::{
    BucketHead, LEVIN_PACKET_REQUEST, LEVIN_PACKET_RESPONSE, LEVIN_PROTOCOL_VER_1, LEVIN_SIGNATURE,
};
use crate::section::Section;
use crate::LevinError;

pub struct Bucket {
    pub head: BucketHead,
    pub body: BytesMut,
}

impl Bucket {
    pub fn create(
        command: u32,
        flags: u32,
        have_to_return_data: bool,
        section: &mut Section,
    ) -> Result<Bucket, LevinError> {
        let mut body = BytesMut::new();
        writeStorageBlockHeader(&mut body);
        section.write(&mut body);
        let bucket_head = BucketHead {
            signature: LEVIN_SIGNATURE,
            cb: body.len() as u64,
            have_to_return_data,
            command,
            return_code: 0,
            flags,
            protocol_version: LEVIN_PROTOCOL_VER_1,
        };
        Ok(Bucket {
            head: bucket_head,
            body: body,
        })
    }

    pub fn create_request(command: u32, section: &mut Section) -> Result<Bucket, LevinError> {
        Bucket::create(command, LEVIN_PACKET_REQUEST, true, section)
    }

    pub fn create_response(command: u32, section: &mut Section) -> Result<Bucket, LevinError> {
        Bucket::create(command, LEVIN_PACKET_RESPONSE, false, section)
    }

    pub fn create_handshake_request() -> Bucket {
        let mut section = Section::handshake_request();
        println!("handshake request section: {:?}", &section);
        Bucket::create_request(1001, &mut section).unwrap()
    }
}

pub const PORTABLE_STORAGE_SIGNATUREA: u32 = 0x01011101;
pub const PORTABLE_STORAGE_SIGNATUREB: u32 = 0x01020101;
pub const PORTABLE_STORAGE_FORMAT_VER: u8 = 1;
pub const PORTABLE_STORAGE_BLOCK_HEADER_LENGTH: usize = 4 + 4 + 1;

fn writeStorageBlockHeader(buf: &mut BytesMut) {
    buf.reserve(PORTABLE_STORAGE_BLOCK_HEADER_LENGTH);
    buf.put_u32_le(PORTABLE_STORAGE_SIGNATUREA);
    buf.put_u32_le(PORTABLE_STORAGE_SIGNATUREB);
    buf.put_u8(PORTABLE_STORAGE_FORMAT_VER);
}

#[cfg(test)]
mod tests {
    use crate::bucket::Bucket;

    #[test]
    fn test_handshake() {
        let b = Bucket::create_handshake_request();
        assert_eq!(b.head.command, 1001);
    }
}
