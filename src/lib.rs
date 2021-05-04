use tokio::net::UdpSocket;
use std::io::{Cursor, Result, Error, ErrorKind};
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

trait ReadPacketExt {
    fn read_cstring(&mut self) -> Result<String>;
    fn read_u8_veccheck(&mut self, src: &Vec<u8>) -> Result<bool>;
}

impl ReadPacketExt for Cursor<Vec<u8>> {
    fn read_cstring(&mut self) -> Result<String> {
        let end = self.get_ref().len() as u64;
        let mut svec = Vec::with_capacity(256);
        while self.position() < end {
            let ch = self.read_u8()?;
            if ch == 0 {
                break;
            } else {
                svec.push(ch);
            }
        }

        Ok(String::from_utf8_lossy(&svec[..]).into_owned())
    }

    fn read_u8_veccheck(&mut self, cmp: &Vec<u8>) -> Result<bool> {
        for cch in cmp {
            let sch = self.read_u8()?;
            if *cch != sch {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

trait WriteCString {
    fn write_cstring(&mut self, src: &str) -> Result<()>;
}

impl WriteCString for Cursor<Vec<u8>> {
    fn write_cstring(&mut self, src: &str) -> Result<()> {
        for ch in src.chars() {
            let mut chu8 = [0; 1];
            ch.encode_utf8(&mut chu8);
            self.write_u8(chu8[0])?;
        }
        self.write_u8(0x00)?;   // 0x00 Terminated
        Ok(())
    }
}

pub struct MSQClient {
    sock: UdpSocket,
}

impl MSQClient {
    pub async fn new() -> Result<MSQClient> {
        let sock = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(MSQClient{
            sock
        })
    }

    pub async fn connect(&mut self, master_server_addr: &str) -> Result<()> {
        self.sock.connect(master_server_addr).await?;
        Ok(())
    }

    pub async fn query(&mut self, filter_str: &str) -> Result<Vec<String>> {
        self.send(filter_str, 0xFF, "0.0.0.0:0").await?;    // First Packet
        let servers = self.recv().await?;
        Ok(servers)
    }

    async fn send(&mut self, filter_str: &str, region_code: u8, address: &str) -> Result<()> {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
        cursor.write_u8(0x31)?;
        cursor.write_u8(region_code)?;
        cursor.write_cstring(address)?;
        cursor.write_cstring(filter_str)?;
        self.sock.send(cursor.get_ref()).await?;
        Ok(())
    }

    async fn recv(&mut self) -> Result<Vec<String>> {
        let mut buf: [u8; 512] = [0x00; 512];
        let len = self.sock.recv(&mut buf).await?;
        let mut cursor = Cursor::new(buf[..len].to_vec());
        let mut servers: Vec<String> = vec![];
        if cursor.read_u8_veccheck(&vec![0xFF, 0xFF, 0xFF, 0xFF, 0x66, 0x0A])? {
            let end = cursor.get_ref().len() as u64;
            while cursor.position() < end {
                let mut addr: [u8; 4] = [0; 4];
                addr[0] = cursor.read_u8()?;
                addr[1] = cursor.read_u8()?;
                addr[2] = cursor.read_u8()?;
                addr[3] = cursor.read_u8()?;
                let port = cursor.read_u16::<BigEndian>()?;
                let addr_str = format!("{}.{}.{}.{}:{}", addr[0], addr[1], addr[2], addr[3], port);

                // If end of IP list
                if addr_str == "0.0.0.0:0" {
                    break;
                }

                servers.push(addr_str);
            }
        } else {
            return Err(Error::new(ErrorKind::Other, "Mismatched starting sequence"));
        }

        Ok(servers)
    }
}

