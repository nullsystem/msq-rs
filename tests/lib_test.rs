use std::io::Result;
use msq::{MSQClient, Region, Filter};

#[tokio::main]
#[test]
async fn test_lib_nt() -> Result<()> {
    let mut client = MSQClient::new().await?;
    client.connect("hl2master.steampowered.com:27011").await?;
    let filter = Filter::new()
        .appid(244630)
        .gameaddr("216.52.143.114");
    println!("{}", filter.as_str());

    let servers = client
        .query(Region::All, filter)
        .await?;

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

    let filter = Filter::new().appid(240)
        .gametype(&vec!["friendlyfire", "alltalk"])
        .nand(1)
            .map("de_dust2");
    println!("{}", filter.as_str());

    let servers = client
        .query(Region::Europe, filter)
        .await?;

    let len = servers.len();
    /*
    for server in servers {
        println!("{}", server);
    }
    */
    println!("Servers: {}", len);
    Ok(())
}
