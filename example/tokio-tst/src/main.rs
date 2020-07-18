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


use std::{error::Error};

use std::sync::{Arc,Mutex};


use tokio::net::TcpStream;
use futures::{future};



#[path = "../../../networking.rs"]
mod networking;
#[path = "../../../structures.rs"]
mod structures;


pub use crate::networking::game_networking::{send,recv};
pub use crate::structures::Structures::SharedVector;




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut r, mut w) = stream.split();

    let sv_v = vec![String::from("Ola"),String::from("a"),String::from("todos"),String::from("os"),String::from("envolvidos")];

    let sharedv = Arc::new(SharedVector {
        svec: Mutex::new(sv_v)
    });

    future::try_join(send(&mut w), recv(&mut r)).await?;
    println!("Returned");
    Ok(())
}

