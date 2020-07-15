//! An example of hooking up stdin/stdout to either a TCP or UDP stream.
//!
//! This example will connect to a socket address specified in the argument list
//! and then forward all data read on stdin to the server, printing out all data
//! received on stdout. An optional `--udp` argument can be passed to specify
//! that the connection should be made over UDP instead of TCP, translating each
//! line entered on stdin to a UDP packet to be sent to the remote address.
//!
//! Note that this is not currently optimized for performance, especially
//! around buffer management. Rather it's intended to show an example of
//! working with a client.
//!
//! This example can be quite useful when interacting with the other examples in
//! this repository! Many of them recommend running this as a simple "hook up
//! stdin/stdout to a server" to get up and running.

#![warn(rust_2018_idioms)]

use futures::StreamExt;
use tokio::io;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};
use tokio::time::Interval;



use std::env;
use std::error::Error;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let addr = addr.parse::<SocketAddr>()?;

    tcp::connect(&addr).await?;
   
    Ok(())
}



mod tcp {
    use std::{error::Error, net::SocketAddr};
    use tokio::net::TcpStream;
    use tokio::io::AsyncWriteExt;
    use tokio::io::AsyncReadExt;
    use bytes::Bytes;
    use futures::{future};
    use tokio::net::tcp::{ReadHalf, WriteHalf};
    use std::io;



    pub async fn connect(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {

        let mut stream = TcpStream::connect(addr).await?;
        let (mut r, mut w) = stream.split();

        let strr = String::from("Hello");

        future::try_join(send(&strr, &mut w), recv(&mut r)).await?;
        println!("Returned");
        Ok(())
    }

    pub async fn send(stri: &str, writer: &mut WriteHalf<'_>,
    ) -> Result<(), io::Error> {
        
        writer.write(&stri.as_bytes()).await?;       
    
        Ok(())
    }
    
    pub async fn recv(reader: &mut ReadHalf<'_>,
    ) -> Result<(), io::Error> {
        loop {
            let mut buf : Vec<u8> = vec![0; 1024];
            let n = reader.read(&mut buf[..]).await?;
    
            let string_tx = String::from_utf8(buf).unwrap();

            if n > 0 {
                println!("{:?}", &string_tx[..n]);
            }
        }
    }
}




/*



mod tcp {
    use std::{error::Error, net::SocketAddr};
    use tokio::net::TcpStream;
    use tokio::io::AsyncWriteExt;


    pub async fn connect(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {

        let mut stream = TcpStream::connect(addr).await?;
        //let (r, w) = stream.split();


/*
  match future::join(sink.send_all(&mut strin), stdout.send_all(&mut strin_r)).await {
            (Err(e), _) | (_, Err(e)) => Err(e.into()),
            _ => Ok(()),
        }
*/

        match stream.write(b"wow").await {
            Err(e) => Err(e.into()),
            _ => Ok(()),
        }
    }
}
*/
