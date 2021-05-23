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

    let servers = client
        .query(Region::Europe,  // Restrict query to Europe region
            Filter::new()       // Create a Filter builder
                .appid(240)     // appid of 240 (CS:S)
                .nand()         // Start of NAND special filter
                    .map("de_dust2")     // Map is de_dust2
                    .empty(true)         // Server is empty
                .end()          // End of NAND special filter
                .gametype(&vec!["friendlyfire", "alltalk"])).await?;
    // Gametype tags of 'friendlyfire' and 'alltalk'

    // nand filter excludes servers that has de_dust2 as
    // its map and is empty

    // nand and nor are both special filters, both closed by
    // using the end method

    Ok(())
}
```

### Blocking version
If you don't want to use async, then a blocking version is available. The
methods functionalities and names should matches its async counterpart.
```rust
use msq::{MSQClientBlock, Region, Filter};
use std::io::Result;

fn main() -> Result<()> {
    let mut client = MSQClientBlock::new()?;
    client.connect("hl2master.steampowered.com:27011")?;
    client.max_servers_on_query(256);

    let servers = client
        .query(Region::Europe,  // Restrict query to Europe region
            Filter::new()       // Create a Filter builder
                .appid(240)     // appid of 240 (CS:S)
                .nand()         // Start of NAND special filter
                    .map("de_dust2")     // Map is de_dust2
                    .empty(true)         // Server is empty
                .end()          // End of NAND special filter
                .gametype(&vec!["friendlyfire", "alltalk"]))?;
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

