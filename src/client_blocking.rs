use crate::filter::Filter;
use crate::region::Region;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::packet_ext::{ReadPacketExt, WritePacketExt};
use std::io::{Cursor, Error, ErrorKind, Result};
use std::net::UdpSocket;

pub struct MSQClientBlock {
    sock: UdpSocket,
    max_servers: usize,
}

impl MSQClientBlock {
    pub fn new() -> Result<Self> {
        let sock = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self {
            sock: sock,
            max_servers: 64,
        })
    }

    pub fn connect(&mut self, master_server_addr: &str) -> Result<()> {
        self.sock.connect(master_server_addr)?;
        Ok(())
    }

    pub fn query_raw(&mut self, region_code: u8, filter_str: &str) -> Result<Vec<String>> {
        self.send(region_code, filter_str, "0.0.0.0:0")?; // First Packet
        Ok(self.recv(region_code, filter_str)?)
    }

    pub fn query(&mut self, region: Region, filter: Filter) -> Result<Vec<String>> {
        Ok(self.query_raw(region.as_u8(), &filter.as_string())?)
    }

    pub fn single_query(master_server: &str, max_servers: usize, region: Region, filter: Filter) -> Result<Vec<String>> {
        let mut client = Self::new()?;
        client.connect(master_server)?;
        client.max_servers_on_query(max_servers);
        client.query(region, filter)
    }

    fn send(&mut self, region_code: u8, filter_str: &str, address: &str) -> Result<()> {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
        cursor.write_u8(0x31)?;
        cursor.write_u8(region_code)?;
        cursor.write_cstring(address)?;
        cursor.write_cstring(filter_str)?;
        self.sock.send(cursor.get_ref())?;
        Ok(())
    }

    fn recv(&mut self, region_code: u8, filter_str: &str) -> Result<Vec<String>> {
        let mut buf: [u8; 2048] = [0x00; 2048];
        let mut servers: Vec<String> = vec![];
        let mut end_of_list = false;
        while !end_of_list {
            let len = self.sock.recv(&mut buf)?;
            let mut cursor = Cursor::new(buf[..len].to_vec());

            if cursor.read_u8_veccheck(&vec![0xFF, 0xFF, 0xFF, 0xFF, 0x66, 0x0A])? {
                let end = cursor.get_ref().len() as u64;
                while cursor.position() < end {
                    let mut addr: [u8; 4] = [0; 4];
                    for i in 0..=3 {
                        addr[i] = cursor.read_u8()?;
                    }
                    let port = cursor.read_u16::<BigEndian>()?;
                    let addr_str =
                        format!("{}.{}.{}.{}:{}", addr[0], addr[1], addr[2], addr[3], port);

                    // If end of IP list
                    if servers.len() >= self.max_servers || addr_str == "0.0.0.0:0" {
                        end_of_list = true;
                        break;
                    }

                    servers.push(addr_str);
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "Mismatched starting sequence"));
            }

            if !end_of_list && servers.len() > 0 {
                self.send(region_code, filter_str, &servers.last().unwrap())?;
            }
        }

        Ok(servers)
    }

    /// Set maximum amount of servers in a given query
    ///
    /// # Arguments
    /// * `max_servers` - Maximum amount of servers in a query
    pub fn max_servers_on_query(&mut self, max_servers: usize) {
        self.max_servers = max_servers;
    }
}

