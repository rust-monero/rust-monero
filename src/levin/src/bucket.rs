use bytes::BytesMut;

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
    fn create(
        command: u32,
        flags: u32,
        have_to_return_data: bool,
        section: &mut Section,
    ) -> Result<Bucket, LevinError> {
        let mut body = BytesMut::new();
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

    fn create_request(command: u32, section: &mut Section) -> Result<Bucket, LevinError> {
        Bucket::create(command, LEVIN_PACKET_REQUEST, true, section)
    }

    fn create_response(command: u32, section: &mut Section) -> Result<Bucket, LevinError> {
        Bucket::create(command, LEVIN_PACKET_RESPONSE, false, section)
    }
}
