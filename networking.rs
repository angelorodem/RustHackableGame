pub mod game_networking {
    use std::{error::Error};

    use std::sync::{Arc,Mutex};

    //use tokio::net::TcpStream;
    //use futures::{future};

    use tokio::io::AsyncWriteExt;
    use tokio::io::AsyncReadExt;

    use tokio::net::tcp::{ReadHalf, WriteHalf};
    use std::io;

    use std::time::Duration;
    use async_std::task;

    use std::io::Cursor;
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    pub use crate::structures::Structures::SharedVector;

    pub fn send_to_server(data : &[u8]){

        
        
    }

    pub fn receive_packet() {
        

    }

    pub fn parse_packets_stream(stream: &mut [u8], bytes_read: usize) -> Vec<Vec<u8>>{

        let mut start: usize = 0;
        let mut next: usize;

        let mut result : Vec<Vec<u8>> = Vec::new();

        println!("Lengh {:?}", bytes_read);
        while  start+4 < bytes_read {

            //println!("Init num: {:?}",start);
            let mut rdr = Cursor::new(stream[start..start+4].to_vec());

            //read_u32().await
            
            next = match ReadBytesExt::read_u32::<BigEndian>(&mut rdr) {
                Ok(t) => {
                    t as usize
                },
                Err(e) => {
                    println!("{:?}",e);
                    0
                }    
            };

            
            result.push(stream[start+4..start+4+next].to_vec());



            let string_tx = String::from_utf8(result[result.len()-1].to_vec()).unwrap();
            println!("{:?}",string_tx);

            start += next + 4;
        }


        result
    }

    pub async fn recv(reader: &mut ReadHalf<'_>) -> Result<(), io::Error> {
        loop {
        
            let mut buf : Vec<u8> = vec![0; 4096];

            //let svector_pointer = &svec_p.clone();
            //let mut vec = svector_pointer.svec.lock().unwrap();
            //println!("Recv Lock");

            let n = match reader.read(&mut buf).await {
                // socket closed 
                Ok(n) if n == 0 => return Ok(()),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return Err(e);
                }
            }; 
            
            parse_packets_stream(&mut buf, n as usize);
        
            //println!("Recv Release");           
        }
        Ok(())
    }


    pub async fn send(writer: &mut WriteHalf<'_>) -> Result<(), io::Error> {
        loop{
             task::sleep(Duration::from_millis(300)).await;
             { 
                 
                 //let svector_pointer = &svec_p.clone();
                 //let mut vec = svector_pointer.svec.lock().unwrap();
                 let a = vec!["AAAA"];
                 //println!("Send Lock");
    
                 for i in a.iter() {
                    let mut byte_array = vec![];
                    let str_bytes = i.as_bytes();
                    WriteBytesExt::write_u32::<BigEndian>(&mut byte_array, str_bytes.len() as u32).unwrap();
                    byte_array.extend(str_bytes);

                    writer.write(&byte_array).await?;
                 }
                 //vec.clear();
    
             }
             //println!("Send Release");
        }
         Ok(())
     }


}