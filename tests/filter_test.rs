use msq::Filter;

#[test]
fn test_filter_simple() {
    let filter = Filter::new()
        .appid(240);

    assert_eq!(filter.as_str(), "\\appid\\240");
}

#[test]
fn test_filter_multi() {
    let filter = Filter::new()
        .appid(240)
        .gametype(&vec!["friendlyfire", "alltalk"])
        .nand(1)
        .map("de_dust2");

    assert_eq!(filter.as_str(), "\\appid\\240\\gametype\\friendlyfire,alltalk\\nand\\1\\map\\de_dust2");
}

