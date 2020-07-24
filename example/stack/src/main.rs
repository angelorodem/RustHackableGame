use std::{error::Error};
use std::sync::{Arc,Mutex};
use tokio::net::TcpStream;
use futures::{future};
use bytes::Bytes;

use std::io;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use async_std::task;
use byteorder::{BigEndian, WriteBytesExt};




//This is the SharedPackets struct that is located in the crate structures
struct SharedPackets {
   data: Mutex<Vec<bytes::Bytes>>
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut r, mut w) = stream.split();


    let mut inc : Vec<bytes::Bytes> = Vec::new();
    inc.push(Bytes::from("Wow"));

    let mut incoming_packets = Arc::new(SharedPackets {
        data: Mutex::new(inc)
    });

    let mut outg : Vec<bytes::Bytes> = Vec::new();
    outg.push(Bytes::from("Wow"));
    let mut outgoint_packets = Arc::new(SharedPackets {
        data: Mutex::new(outg)
    });

    let mut local_incoming_packets = Arc::clone(&incoming_packets);
    let mut local_outgoint_packets = Arc::clone(&outgoint_packets);
    let mut rarc = Arc::new(Mutex::new(r));
    let mut warc = Arc::new(Mutex::new(w));

    tokio::spawn(async move {
 

        //send and receive are both async functions that contain an infinite loop
        //they basically use AsyncWriteExt and AsyncReadExt to manipulate both halves of the stream
        //send reads the queue and write this data on the socket
        //recv reads the socket and write this data on the queue
        //both "queues" are manipulated by the main thread
        let mut read = &*rarc.lock().unwrap();
        let mut write = &*warc.lock().unwrap();

        future::try_join(send(&mut write, &mut local_outgoint_packets), recv(&mut read, &mut local_incoming_packets)).await;
    });

   
    loop {
        //read & write other stuff on both incoming_packets & outgoint_packets
        //until the end of the program
    }
}


async fn recv(reader: &mut ReadHalf<'_>, queue: &mut Arc<SharedPackets>) -> Result<(), io::Error> {

    let mut buf : Vec<u8> = vec![0; 4096];

    let n = match reader.read(&mut buf).await {
        Ok(n) if n == 0 => return Ok(()),
        Ok(n) => n,
        Err(e) => {
            eprintln!("failed to read from socket; err = {:?}", e);
            return Err(e);
        }
    };                   
    Ok(())
}



async fn send(writer: &mut WriteHalf<'_>, queue: &mut Arc<SharedPackets>) -> Result<(), io::Error> {     
    let a = vec!["AAAA"];
    for i in a.iter() {
        let mut byte_array = vec![];
        let str_bytes = i.as_bytes();
        WriteBytesExt::write_u32::<BigEndian>(&mut byte_array, str_bytes.len() as u32).unwrap();
        byte_array.extend(str_bytes);

        writer.write(&byte_array).await?;
    }
    Ok(())
}