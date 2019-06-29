use std::thread;
use std::thread::Thread;
use std::time::Duration;

use bytes::{Buf, LittleEndian};
use bytes::BytesMut;
use bytes::IntoBuf;
use tokio::net::TcpStream;
use tokio::prelude::*;

use levin::bucket::Bucket;
use levin::bucket_head::{BucketHead, LEVIN_BUCKET_HEAD_LENGTH};
use levin::section::Section;
use std::fs::read;

fn find_other_node() {
    let addr = "116.88.75.110:18080".parse().unwrap();

    let client = TcpStream::connect(&addr)
        .and_then(|mut stream| {
            let (mut reader, mut writer) = stream.split();
            println!("created stream");
            let bucket = Bucket::create_handshake_request();
            let bucket_head = bucket.head;
            println!("bucket_head: {:?}", &bucket_head);
            println!(
                "bucket_body length: {:?}, detail:{:?}",
                &bucket.body.len(),
                &bucket.body
            );
            let bucket_body = bucket.body;
            let mut b = BytesMut::new();
            BucketHead::write(&bucket_head, &mut b);
            writer.write_all(&b.to_vec());
            writer.write_all(&bucket_body.to_vec());
            thread::sleep(Duration::from_millis(3000));
            let mut buf = vec![0u8; LEVIN_BUCKET_HEAD_LENGTH];
            let result = reader
                .read_exact(&mut buf)
                .and_then(|r| {
                    let mut head_bytes = BytesMut::from(buf);
                    let new_head = BucketHead::read(&mut head_bytes.into_buf());
                    println!("new_head = {:?}", &new_head);
                    let size = new_head.unwrap().cb;
                    thread::sleep_ms(3000);
                    let mut  a = vec![0 as u8; 4];
                    reader.read_exact(&mut a);
                    println!("{:?}", a);
                    let mut  b = vec![0 as u8; 4];
                    reader.read_exact(&mut b);
                    println!("{:?}", b);
                    let mut  c = vec![0 as u8; 1];
                    reader.read_exact(&mut c);
                    println!("{:?}", c);

                    let mut bytes_array = vec![0 as u8; size as usize];
                    reader.read_exact(&mut bytes_array);
                    println!("-------");
                    for i in &bytes_array {
                        println!("{}", i);
                    }
                    println!("-------");
                    let mut buf = BytesMut::from(bytes_array).into_buf();
                    println!("recieved buf: {:?}", &buf);
                    let section = Section::read(&mut buf);
                    println!("recieved data: {:?}", section);
                    println!("finished");
                    Ok(())
                })
                .map_err(|e| {
                    println!("error: {:?}", e);
                });
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
    use std::io::Cursor;

    use bytes::{Buf, BufMut, Bytes, BytesMut};

    use levin::bucket::Bucket;
    use levin::bucket_head::BucketHead;

    use crate::peer_retreiver::find_other_node;

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
