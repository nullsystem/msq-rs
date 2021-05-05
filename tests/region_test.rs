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

