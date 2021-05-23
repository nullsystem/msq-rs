//! msq is a rust library implementation of the legacy [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol).
//!
//! ## Quick Start
//! ```rust
//! use msq::{MSQClient, Region, Filter};
//! use std::io::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Startup the client
//!     let mut client = MSQClient::new().await?;
//!
//!     // Connect to the master server
//!     client.connect("hl2master.steampowered.com:27011").await?;
//!
//!     // Maximum amount of servers we wanted to query
//!     client.max_servers_on_query(256);
//!
//!    let servers = client
//!        .query(Region::Europe,  // Restrict query to Europe region
//!            Filter::new()       // Create a Filter builder
//!                .appid(240)     // appid of 240 (CS:S)
//!                .nand()         // Start of NAND special filter
//!                    .map("de_dust2")     // Map is de_dust2
//!                    .empty(true)         // Server is empty
//!                .end()          // End of NAND special filter
//!                .gametype(&vec!["friendlyfire", "alltalk"]))
//!                // Gametype tags of 'friendlyfire' and 'alltalk'
//!
//!                // nand filter excludes servers that has de_dust2 as
//!                // its map and is empty
//!        .await?;
//!
//!    // nand and nor are both special filters, both closed by
//!    // using the end method
//!
//!     Ok(())
//! }
//! ```

pub mod filter;
pub mod region;

pub use crate::filter::Filter;
pub use crate::region::Region;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Error, ErrorKind, Result};
use tokio::net::UdpSocket;

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

trait WritePacketExt {
    fn write_cstring(&mut self, src: &str) -> Result<()>;
}

impl WritePacketExt for Cursor<Vec<u8>> {
    fn write_cstring(&mut self, src: &str) -> Result<()> {
        for ch in src.chars() {
            let mut chu8 = [0; 1];
            ch.encode_utf8(&mut chu8);
            self.write_u8(chu8[0])?;
        }
        self.write_u8(0x00)?; // 0x00 Terminated
        Ok(())
    }
}

/// The primary MSQ client driver, refer to the Quick Start section
/// on using this.
pub struct MSQClient {
    sock: UdpSocket,
    max_servers: usize,
}

impl MSQClient {
    /// Create a new MSQClient variable and binds the UDP socket to 0.0.0.0:0
    pub async fn new() -> Result<MSQClient> {
        let sock = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(MSQClient {
            sock: sock,
            max_servers: 64,
        })
    }

    /// Connect the client to the given master server address/hostname
    ///
    /// # Arguments
    /// * `master_server_addr` - The master server's hostname/ip address
    ///
    /// # Example
    /// ```
    /// use msq::MSQClient;
    /// use std::io::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let mut client = MSQClient::new().await?;
    ///     client.connect("hl2master.steampowered.com:27011").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(&mut self, master_server_addr: &str) -> Result<()> {
        self.sock.connect(master_server_addr).await?;
        Ok(())
    }

    /// Query with raw bytes
    ///
    /// # Arguments
    /// * `region_code` - Region code in u8 (`0x00 - 0x07 / 0xFF`)
    /// * `filter_str` - Filter in plain string (EX: `\\appid\\240\\map\\de_dust2`)
    pub async fn query_raw(&mut self, region_code: u8, filter_str: &str) -> Result<Vec<String>> {
        self.send(region_code, filter_str, "0.0.0.0:0").await?; // First Packet
        Ok(self.recv(region_code, filter_str).await?)
    }

    /// Query with specified Region and Filter
    ///
    /// # Arguments
    /// * `region` - [Region] enum (`Region::USEast` - `Region::Africa` / `Region::All`)
    /// * `filter` - [Filter] builder (EX: `Filter::new().appid(240).map("de_dust2")`)
    pub async fn query(&mut self, region: Region, filter: Filter) -> Result<Vec<String>> {
        Ok(self.query_raw(region.as_u8(), &filter.as_string()).await?)
    }

    async fn send(&mut self, region_code: u8, filter_str: &str, address: &str) -> Result<()> {
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
        cursor.write_u8(0x31)?;
        cursor.write_u8(region_code)?;
        cursor.write_cstring(address)?;
        cursor.write_cstring(filter_str)?;
        self.sock.send(cursor.get_ref()).await?;
        Ok(())
    }

    async fn recv(&mut self, region_code: u8, filter_str: &str) -> Result<Vec<String>> {
        let mut buf: [u8; 2048] = [0x00; 2048];
        let mut servers: Vec<String> = vec![];
        let mut end_of_list = false;
        while !end_of_list {
            let len = self.sock.recv(&mut buf).await?;
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
                self.send(region_code, filter_str, &servers.last().unwrap())
                    .await?;
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
