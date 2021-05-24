use crate::filter::Filter;
use crate::region::Region;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use crate::packet_ext::{ReadPacketExt, WritePacketExt};
use std::io::{Cursor, Error, ErrorKind, Result};
use tokio::net::UdpSocket;

/// The primary MSQ client driver (async)
///
/// * Requires feature: `async` (Turned **on** by default)
/// * Intended to be used with [`Filter`] and [`Region`].
/// * This uses the [`tokio`] asynchronous UDP Socket to achieve an
/// async MSQ client driver.
/// * The non-async/blocking version of this: [`MSQClientBlock`](crate::MSQClientBlock)
///
/// ## Quick Start
/// ```rust
/// use msq::{MSQClient, Region, Filter};
/// use std::io::Result;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let mut client = MSQClient::new().await?;
///     client.connect("hl2master.steampowered.com:27011").await?;
///     client.max_servers_on_query(256);
///
///     let servers = client
///         .query(Region::Europe,  // Restrict query to Europe region
///             Filter::new()       // Create a Filter builder
///                 .appid(240)     // appid of 240 (CS:S)
///                 .nand()         // Start of NAND special filter
///                     .map("de_dust2")     // Map is de_dust2
///                     .empty(true)         // Server is empty
///                 .end()          // End of NAND special filter
///                 .gametype(&vec!["friendlyfire", "alltalk"])).await?;
///     Ok(())
/// }
/// ```
pub struct MSQClient {
    sock: UdpSocket,
    max_servers: usize,
}

impl MSQClient {
    /// Create a new MSQClient variable and binds the UDP socket to `0.0.0.0:0`
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
    /// Returns a Vec list of IP addresses in strings
    ///
    /// # Arguments
    /// * `region` - [`Region`] enum (`Region::USEast` - `Region::Africa` / `Region::All`)
    /// * `filter` - [`Filter`] builder (EX: `Filter::new().appid(240).map("de_dust2")`)
    pub async fn query(&mut self, region: Region, filter: Filter) -> Result<Vec<String>> {
        Ok(self.query_raw(region.as_u8(), &filter.as_string()).await?)
    }

    /// Do a single query in one function
    ///
    /// # Arguments
    /// * `master_server` - The address of the master server to fetch the query from
    /// * `max_servers` - The maximum amount of servers to query
    /// * `region` - [`Region`] enum (`Region::USEast` - `Region::Africa` / `Region::All`)
    /// * `filter` - [`Filter`] builder (EX: `Filter::new().appid(240).map("de_dust2")`)
    ///
    /// # Example
    /// ```
    /// use msq::{MSQClient, Region, Filter};
    /// use std::io::Result;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let servers_list = MSQClient::single_query(
    ///             "hl2master.steampowered.com:27011",
    ///             256,
    ///             Region::Europe,
    ///             Filter::new().appid(240)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn single_query(master_server: &str, max_servers: usize, region: Region, filter: Filter) -> Result<Vec<String>> {
        let mut client = Self::new().await?;
        client.connect(master_server).await?;
        client.max_servers_on_query(max_servers);
        client.query(region, filter).await
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
