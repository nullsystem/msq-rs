use msq::Region;

#[test]
fn test_region_codes() {
    assert_eq!(Region::USEast.as_u8(), 0x00);
    assert_eq!(Region::USWest.as_u8(), 0x01);
    assert_eq!(Region::SouthAmerica.as_u8(), 0x02);
    assert_eq!(Region::Europe.as_u8(), 0x03);
    assert_eq!(Region::Asia.as_u8(), 0x04);
    assert_eq!(Region::Australia.as_u8(), 0x05);
    assert_eq!(Region::MiddleEast.as_u8(), 0x06);
    assert_eq!(Region::Africa.as_u8(), 0x07);
    assert_eq!(Region::All.as_u8(), 0xFF);
}

#[test]
fn test_region_codes_from() {
    assert_eq!(Region::from_u8(0x00).unwrap(), Region::USEast);
    assert_eq!(Region::from_u8(0x01).unwrap(), Region::USWest);
    assert_eq!(Region::from_u8(0x02).unwrap(), Region::SouthAmerica);
    assert_eq!(Region::from_u8(0x03).unwrap(), Region::Europe);
    assert_eq!(Region::from_u8(0x04).unwrap(), Region::Asia);
    assert_eq!(Region::from_u8(0x05).unwrap(), Region::Australia);
    assert_eq!(Region::from_u8(0x06).unwrap(), Region::MiddleEast);
    assert_eq!(Region::from_u8(0x07).unwrap(), Region::Africa);
    assert_eq!(Region::from_u8(0xFF).unwrap(), Region::All);
    for i in 0x08..=0xFE {
        assert_eq!(Region::from_u8(i).is_err(), true);
    }
}
