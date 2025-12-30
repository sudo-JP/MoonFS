use std::net::{TcpListener, TcpStream};
use std::io::Read;
use anyhow::Result;

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer{
    pub fn new(addr: &str) -> Result<Self>{
        let listener = TcpListener::bind(addr)?;
        Ok(Self { listener })
    }

    pub fn accept_and_read(&self) -> Result<Vec<u8>>{
        let (mut stream, addr) = self.listener.accept()?;
        println!("Accepted connection: {}", addr);

        let mut buffer = Vec::new();
        let mut chunk = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut chunk)?;
            if bytes_read == 0 {
                break;  // Connection closed
            }
            buffer.extend_from_slice(&chunk[..bytes_read]);
        }

        Ok(buffer)
    }
}