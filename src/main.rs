use tokio::net::UdpSocket;
use std::io::Cursor;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

trait ReadPacketExt {
    fn read_cstring(&mut self) -> std::io::Result<String>;
    fn read_u8_veccheck(&mut self, src: &Vec<u8>) -> std::io::Result<bool>;
}

impl ReadPacketExt for Cursor<Vec<u8>> {
    fn read_cstring(&mut self) -> std::io::Result<String> {
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

    fn read_u8_veccheck(&mut self, cmp: &Vec<u8>) -> std::io::Result<bool> {
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
    fn write_cstring(&mut self, src: &str) -> std::io::Result<()>;
}

impl WriteCString for Cursor<Vec<u8>> {
    fn write_cstring(&mut self, src: &str) -> std::io::Result<()> {
        for ch in src.chars() {
            let mut chu8 = [0; 1];
            ch.encode_utf8(&mut chu8);
            self.write_u8(chu8[0])?;
        }
        self.write_u8(0x00)?;   // 0x00 Terminated
        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let master_server_address = "hl2master.steampowered.com:27011";
    sock.connect(master_server_address).await?;
    let mut buf: [u8; 512] = [0x00; 512];

    // Construct first req
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
    cursor.write_u8(0x31)?;
    cursor.write_u8(0xFF)?;
    cursor.write_cstring("0.0.0.0:0")?;
    cursor.write_cstring("\\appid\\244630")?;
    sock.send(cursor.get_ref()).await?;

    // Receive the packet(s)
    let len = sock.recv(&mut buf).await?;
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
        eprintln!("Mismatched starting sequence");
    }

    for server in servers {
        println!("{}", server);
    }

    Ok(())
}
