#[allow(non_snake_case, dead_code, unused_imports)]

#[path = "../../networking.rs"]
mod networking;
#[path = "../../structures.rs"]
mod structures;

pub use crate::networking::game_networking::{recv, send, parse_packets_stream};
pub use crate::structures::Structures::SharedVector;

use std::sync::{Arc,Mutex};

use futures::SinkExt;
use tokio::net::TcpStream;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, BytesCodec};

use tokio::net::TcpListener;
use tokio::prelude::*;


use bytes::Bytes;

use std::time::Duration;
use async_std::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:8080").await?;
    // println!("Listening on: {}", addr);

    let mut init_vector = Vec::new();
    init_vector.push("hello".to_string());
    let sharedv = Arc::new(SharedVector {
        svec: Mutex::new(init_vector),
    });


    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {      

                let mut shared_data = sharedv.clone();
                

                tokio::spawn(async move {           
                    let mut bytes = Framed::new(socket, BytesCodec::new());
                    
                    while let Some(data) = bytes.next().await {
                        match data {
                            Ok(mut byte) => {
                                //println!("{:?}",byte);
                                let n = byte.len();
                                parse_packets_stream(&mut byte[..],n);

                                if let Err(e) = bytes.send(Bytes::from(byte)).await {
                                    println!("error on sending response; error = {:?}", e);
                                }


                            },
                            Err(e) => println!("{:?}",e),
                        }
                    }

                    
                });

            },
            Err(e) => println!("Error reading socket {:?}", e),

        }
        task::sleep(Duration::from_millis(2)).await;
    }
}

//https://github.com/rust-lang/futures-rs/issues/1906




fn handle_request(){
    
}