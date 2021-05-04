#[tokio::main]
#[test]
async fn test_lib() -> std::io::Result<()> {
    let mut client = msq::MSQClient::new().await?;
    client.connect("hl2master.steampowered.com:27011").await?;
    let servers = client
        .query(msq::Region::All,
            msq::Filter::new()
                .appid(244630)
        )
        .await?;

    println!("Servers: {}", servers.len());
    for server in servers {
        println!("{}", server);
    }
    Ok(())
}
