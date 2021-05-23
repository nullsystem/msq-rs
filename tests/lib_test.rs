use msq::{Filter, MSQClient, Region};
use std::io::Result;

#[tokio::main]
#[test]
async fn test_lib_nt() -> Result<()> {
    let mut client = MSQClient::new().await?;
    client.connect("hl2master.steampowered.com:27011").await?;
    let filter = Filter::new().appid(244630).gameaddr("216.52.143.114");
    println!("{}", filter.as_string());

    let servers = client.query(Region::All, filter).await?;

    println!("Servers: {}", servers.len());
    for server in servers {
        println!("{}", server);
    }
    Ok(())
}

#[tokio::main]
#[test]
async fn test_lib_css() -> Result<()> {
    let mut client = MSQClient::new().await?;
    client.connect("hl2master.steampowered.com:27011").await?;
    client.max_servers_on_query(256);

    let filter = Filter::new()
        .appid(240)
        .gametype(&vec!["friendlyfire", "alltalk"])
        .nand()
        .map("de_dust2")
        .end();
    println!("{}", filter.as_string());

    let servers = client.query(Region::Europe, filter).await?;

    let len = servers.len();
    /*
    for server in servers {
        println!("{}", server);
    }
    */
    println!("Servers: {}", len);
    Ok(())
}

#[tokio::main]
#[test]
async fn test_lib_css_big_query() -> Result<()> {
    let mut client = MSQClient::new().await?;
    client.connect("hl2master.steampowered.com:27011").await?;
    client.max_servers_on_query(4096);
    let servers = client.query(Region::All, Filter::new().appid(240)).await?;
    let len = servers.len();
    println!("Servers: {}", len);
    Ok(())
}

#[tokio::main]
#[test]
async fn test_lib_css_no_query() -> Result<()> {
    let mut client = MSQClient::new().await?;
    client.connect("hl2master.steampowered.com:27011").await?;
    client.max_servers_on_query(0);
    let servers = client.query(Region::All, Filter::new().appid(240)).await?;
    assert_eq!(servers.len(), 0);
    Ok(())
}
