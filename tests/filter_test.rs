use msq::Filter;

#[test]
fn test_filter_simple() {
    let filter = Filter::new().appid(240);

    assert_eq!(filter.as_string(), "\\appid\\240");
}

#[test]
fn test_filter_multi() {
    let filter = Filter::new()
        .appid(240)
        .gametype(&vec!["friendlyfire", "alltalk"])
        .nand()
        .map("de_dust2")
        .end();

    assert_eq!(
        filter.as_string(),
        "\\appid\\240\\gametype\\friendlyfire,alltalk\\nand\\1\\map\\de_dust2"
    );
}

#[test]
fn test_filter_multi_2() {
    let filter = Filter::new()
        .appid(240)
        .nand()
        .map("de_dust2")
        .end()
        .gametype(&vec!["friendlyfire", "alltalk"]);

    assert_eq!(
        filter.as_string(),
        "\\appid\\240\\nand\\1\\map\\de_dust2\\gametype\\friendlyfire,alltalk"
    );
}

#[test]
fn test_filter_multi_3() {
    let filter = Filter::new()
        .appid(240)
        .nand()
        .map("de_dust2")
        .gametype(&vec!["friendlyfire", "alltalk"])
        .end();

    assert_eq!(
        filter.as_string(),
        "\\appid\\240\\nand\\2\\map\\de_dust2\\gametype\\friendlyfire,alltalk"
    );
}

#[test]
fn test_filter_empty_gametype_list() {
    let filter = Filter::new()
        .appid(240)
        .nand()
        .map("de_dust2")
        .gametype(&vec![])
        .end();

    assert_eq!(
        filter.as_string(),
        "\\appid\\240\\nand\\1\\map\\de_dust2"
    );
}

#[test]
fn test_filter_gametype2() {
    let filter = Filter::new()
        .appid(440)
        .gametype(&vec!["cp"])
        .nor()
            .gametype(&vec!["alltalk"])
        .end();

    assert_eq!(
        filter.as_string(),
        "\\appid\\440\\gametype\\cp\\nor\\1\\gametype\\alltalk"
    );
}

