#[cfg(feature = "non-async")]
use msq::{MSQClientBlock, Filter, Region};
#[cfg(feature = "non-async")]
use std::io::Result;

#[cfg(feature = "non-async")]
#[test]
fn test_lib_noasync_nt() -> Result<()> {
    let mut client = MSQClientBlock::new()?;
    client.connect("hl2master.steampowered.com:27011")?;
    let filter = Filter::new().appid(244630).gameaddr("216.52.143.114");
    println!("{}", filter.as_string());

    let servers = client.query(Region::All, filter)?;

    println!("Servers: {}", servers.len());
    for server in servers {
        println!("{}", server);
    }
    Ok(())
}

#[cfg(feature = "non-async")]
#[test]
fn test_lib_noasync_css() -> Result<()> {
    let mut client = MSQClientBlock::new()?;
    client.connect("hl2master.steampowered.com:27011")?;
    client.max_servers_on_query(256);

    let filter = Filter::new()
        .appid(240)
        .gametype(&vec!["friendlyfire", "alltalk"])
        .nand()
        .map("de_dust2")
        .end();
    println!("{}", filter.as_string());

    let servers = client.query(Region::Europe, filter)?;

    let len = servers.len();
    /*
    for server in servers {
        println!("{}", server);
    }
    */
    println!("Servers: {}", len);
    Ok(())
}

#[cfg(feature = "non-async")]
#[test]
fn test_lib_noasync_css_big_query() -> Result<()> {
    let mut client = MSQClientBlock::new()?;
    client.connect("hl2master.steampowered.com:27011")?;
    client.max_servers_on_query(4096);
    let servers = client.query(Region::All, Filter::new().appid(240))?;
    let len = servers.len();
    println!("Servers: {}", len);
    Ok(())
}

#[cfg(feature = "non-async")]
#[test]
fn test_lib_noasync_css_no_query() -> Result<()> {
    let mut client = MSQClientBlock::new()?;
    client.connect("hl2master.steampowered.com:27011")?;
    client.max_servers_on_query(0);
    let servers = client.query(Region::All, Filter::new().appid(240))?;
    assert_eq!(servers.len(), 0);
    Ok(())
}

