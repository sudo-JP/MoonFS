use std::{io::Write, net::TcpStream};
use anyhow::{Result};


pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub fn new(addr: String) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        let mut byte_sent: usize = 0;
        while byte_sent < buf.len() {
            byte_sent += self.stream.write(buf)?;
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        self.stream.shutdown(std::net::Shutdown::Write)?;
        Ok(())
    }
}

