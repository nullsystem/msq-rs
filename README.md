# msq-rs
Rust library implementation of the legacy [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol).

## License
msq-rs is released under the [MIT License](LICENSE)

## Dependencies
* [tokio](https://tokio.rs/)
* [byteorder](https://github.com/BurntSushi/byteorder)

## Instructions
### Direct from the repository
* Add the following in `Cargo.toml`:
* `msq = { git = "https://github.com/mtcw99/msq-rs.git" }`

### crates.io
* Coming soon

## Quick Start
```rust
// Startup the client
let mut client = MSQClient::new().await?;

// Connect to the master server
client.connect("hl2master.steampowered.com:27011").await?;

// Maximum amount of servers we wanted to query
client.max_servers_on_query(256);

// Do a query, which is restricted to the Europe region
// and filter by appid 240 (CS:S), gametype has to have
// friendlyfire and alltalk tags on it, and its current map
// that is not de_dust2
let servers = client
    .query(Region::Europe,
        Filter::new().appid(240)
            .gametype(&vec!["friendlyfire", "alltalk"])
            .nand(1)
                .map("de_dust2"))
    .await?;
```

