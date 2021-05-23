/// Region enum to restrict the servers region the query searches for
///
pub enum Region {
    USEast,
    USWest,
    SouthAmerica,
    Europe,
    Asia,
    Australia,
    MiddleEast,
    Africa,
    All, // Rest of the world
}

impl Region {
    /// Return raw u8 byte code of its specified region
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::USEast => 0x00,
            Self::USWest => 0x01,
            Self::SouthAmerica => 0x02,
            Self::Europe => 0x03,
            Self::Asia => 0x04,
            Self::Australia => 0x05,
            Self::MiddleEast => 0x06,
            Self::Africa => 0x07,
            Self::All => 0xFF,
        }
    }
}
