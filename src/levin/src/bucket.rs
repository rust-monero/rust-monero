use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::bucket_head::{BucketHead, LEVIN_PACKET_REQUEST, LEVIN_PROTOCOL_VER_1, LEVIN_SIGNATURE};
use crate::LevinError;
use crate::section::Section;

pub struct Bucket {
    pub head: BucketHead,
    pub body: BytesMut,
}


impl Bucket {
    fn create(command: u32, section: &mut Section) -> Result<Bucket, LevinError> {
        let mut body = BytesMut::new();
        section.write(&mut body);
        let bucket_head = BucketHead {
            signature: LEVIN_SIGNATURE,
            cb: body.len() as u64,
            have_to_return_data: true,
            command,
            return_code: 0,
            flags: LEVIN_PACKET_REQUEST,
            protocol_version: LEVIN_PROTOCOL_VER_1,
        };
        Ok(Bucket {
            head: bucket_head,
            body: body
        })
    }
}