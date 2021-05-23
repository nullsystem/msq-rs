use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Result};

pub trait ReadPacketExt: ReadBytesExt {
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

pub trait WritePacketExt: WriteBytesExt {
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

