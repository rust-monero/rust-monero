use bytes::Buf;
use bytes::BytesMut;
use bytes::IntoBuf;
use tokio::net::TcpStream;
use tokio::prelude::*;

use levin::bucket::Bucket;
use levin::bucket_head::{BucketHead, LEVIN_BUCKET_HEAD_LENGTH};

fn find_other_node() {
    let addr = "116.88.75.110:18080".parse().unwrap();

    let client = TcpStream::connect(&addr)
        .and_then(|mut stream| {
            println!("created stream");
            let bucket = Bucket::create_handshake_request();
            let bucket_head = bucket.head;
            println!("bucket_head: {:?}", &bucket_head);
            println!("bucket_body: {:?}", &bucket.body);
            let bucket_body = bucket.body;
            let mut b = BytesMut::new();
            BucketHead::write(&bucket_head, &mut b);
            stream.write_all(&b.to_vec());
            stream.write_all(&bucket_body.to_vec());
            let mut buf = vec![0u8; LEVIN_BUCKET_HEAD_LENGTH];
            stream.read_exact(&mut buf);
            let mut head_bytes = BytesMut::from(buf);
            let new_head = BucketHead::read(&mut head_bytes.into_buf());
            println!("new_head = {:?}", &new_head);
            println!("finished");
            Ok(())
        })
        .map_err(|e| {
            println!("connection error = {:?}", e);
        });
    println!("About to create the stream and write to it...");
    tokio::run(client);
    println!("Stream has been created and written to.");
}


#[cfg(test)]
mod tests {
    use crate::peer_retreiver::find_other_node;
    use std::io::Cursor;
    use levin::bucket_head::BucketHead;
    use bytes::{Buf, BufMut, Bytes, BytesMut};
    use levin::bucket::Bucket;

    #[test]
    fn test_send() {
        find_other_node();
    }

    #[test]
    fn test_read() {
        let bucket = Bucket::create_handshake_request();
        let mut bucket_head = &bucket.head;
        let mut body = &bucket.body;
        let mut b = BytesMut::new();
        bucket_head.write(&mut b);
        println!("{:?}", &b);
        println!("{:?}", body.len());
    }
}