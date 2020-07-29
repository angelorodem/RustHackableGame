#[allow(non_snake_case, dead_code, unused_imports)]
pub mod game_networking {
    use std::{error::Error};

    use std::sync::{Arc};
    use tokio::sync::Mutex;

    //use tokio::net::TcpStream;
    //use futures::{future};

    use tokio::io::AsyncWriteExt;
    use tokio::io::AsyncReadExt;
    use tokio::time::delay_for;

    use tokio::net::tcp::{ReadHalf, WriteHalf};
    use std::io;

    use std::time::Duration;
    use async_std::task;

    use std::io::Cursor;
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

    use bytes::{Bytes};

    use internet_checksum::checksum;    

    pub use crate::structures::Structures;


    pub async fn parse_packets_stream(stream: &mut [u8], bytes_read: usize, mut packetsw : &mut Arc<Structures::IncomingPackets>) -> Result<(), ()> {

        let mut start: usize = 0;
        let mut next: usize;        

        while  start+4 < bytes_read {
            let mut rdr = Cursor::new(stream[start..start+4].to_vec());            
            next = match ReadBytesExt::read_u32::<BigEndian>(&mut rdr) {
                Ok(t) => {
                    t as usize
                },
                Err(e) => {
                    println!("{:?}",e);
                    0
                }    
            }; 

            if next >= bytes_read ||  start+4+next > bytes_read {
                return Err(());
            }

            let mut packets = packetsw.data.lock().await;
            packets.push(Bytes::from(stream[start+4..start+4+next].to_vec()));    

            start += next + 4;
        }
        Ok(())
    }


    pub fn parse_packets_stream_sync(stream: &mut [u8], bytes_read: usize, mut packets : &mut Structures::Packets) -> Result<(), ()> {

        let mut start: usize = 0;
        let mut next: usize;        

        while  start+4 < bytes_read {
            let mut rdr = Cursor::new(stream[start..start+4].to_vec());            
            next = match ReadBytesExt::read_u32::<BigEndian>(&mut rdr) {
                Ok(t) => {
                    t as usize
                },
                Err(e) => {
                    println!("{:?}",e);
                    0
                }    
            }; 

            if next >= bytes_read ||  start+4+next > bytes_read {
                return Err(());
            }

            packets.push(Bytes::from(stream[start+4..start+4+next].to_vec()));    

            start += next + 4;
        }
        Ok(())
    }


    pub async fn recv(readerw: Arc<Mutex<&mut ReadHalf<'_>>>, mut packetsw : &mut Arc<Structures::IncomingPackets>) -> Result<(), io::Error> {
        
        let mut reader = readerw.lock().await;
        let mut buf : Vec<u8> = vec![0; 4096];
        loop {
            delay_for(Duration::from_millis(10)).await;

            buf.iter_mut().for_each(|x| *x = 0);

            let n = match reader.read(&mut buf).await {
                // socket closed 
                Ok(n) if n == 0 => return Ok(()),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return Err(e);
                }
            }; 
            
            if let Err(e) = parse_packets_stream(&mut buf, n as usize, &mut packetsw).await {
                println!("{:?}",e);
            }
        }
        
        Ok(())
    }

    pub fn prepare_data(data : &bytes::Bytes) -> bytes::Bytes{
        let mut byte_array = vec![];

        WriteBytesExt::write_u32::<BigEndian>(&mut byte_array, (data.len()+2) as u32).unwrap();
        let chk_sum = checksum(&data);
        byte_array.extend(data);
        byte_array.extend(&chk_sum);

        bytes::Bytes::from(byte_array)
    }

    pub async fn send(writerw: Arc<Mutex<&mut WriteHalf<'_>>>, packetsw: &mut Arc<Structures::OutgoingPackets>) -> Result<(), io::Error> {    
        
        let mut writer = writerw.lock().await;
        loop {
            delay_for(Duration::from_millis(10)).await;
            let mut packets = packetsw.data.lock().await;
            for mut packet in &mut packets.iter() {
                let mut packet_ready = prepare_data(packet);
                writer.write(&mut packet_ready).await?;
            }  
            packets.clear();
        }

        Ok(())
    }


}