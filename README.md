# msq-rs
Rust library implementation of the legacy [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol).

* [Documentation](https://docs.rs/msq/)
* [crates.io](https://crates.io/crates/msq)
* [Repository](https://github.com/mtcw99/msq-rs)
* [Release Notes](https://github.com/mtcw99/msq-rs/releases)

## Usage
Add this to your `Cargo.toml`:
```
[dependencies]
msq = "0.1"
```
To get started using msq, see the Quick Start section below
and take a look at the [documentation](https://docs.rs/msq/).

## Quick Start
```rust
use msq::{MSQClient, Region, Filter};
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Startup the client
    let mut client = MSQClient::new().await?;

    // Connect to the master server
    client.connect("hl2master.steampowered.com:27011").await?;
 
    // Maximum amount of servers we wanted to query
    client.max_servers_on_query(256);

    // Do a query, which is restricted to the Europe region
    // and filter by appid 240 (CS:S), maps that are not
    // de_dust2, and gametype tags of friendlyfire and alltalk
    let servers = client
        .query(Region::Europe,
            Filter::new().appid(240)
                .nand()
                    .map("de_dust2")
                .end()
                .gametype(&vec!["friendlyfire", "alltalk"]))
        .await?;
    Ok(())
}
```

## License
msq-rs is released under the [MIT License](LICENSE)

## Dependencies
* [tokio](https://tokio.rs/)
* [byteorder](https://github.com/BurntSushi/byteorder)

## Misc
The following library goes well with this one:
* Source A2S Queries: [a2s-rs](https://github.com/rumblefrog/a2s-rs) 

