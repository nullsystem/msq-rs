//! msq is a rust library implementation of the legacy [Master Server Query Protocol](https://developer.valvesoftware.com/wiki/Master_Server_Query_Protocol).
//!
//! # Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! msq = "0.2"
//! ```
//! If you want to get straight from the latest master branch:
//! ```toml
//! [dependencies]
//! msq = { git = "https://github.com/nullsystem/msq-rs.git" }
//! ```
//! 
//! To get started using msq, see the [Quick Start](#quick-start) section below.
//! 
//! ## Features
//! By default, both async [`MSQClient`] and non-async/blocking [`MSQClientBlock`] are included.
//! However, if you want to include either only async or only non-async, you could do the following:
//! 
//! * For async/[`MSQClient`] **only**:
//! ```toml
//! [dependencies]
//! msq = { version = "0.2", default-features = false, features = ["async"] }
//! ```
//! * For non-async/[`MSQClientBlock`] **only**:
//! ```toml
//! [dependencies]
//! msq = { version = "0.2", default-features = false, features = ["non-async"] }
//! ```
//! 
//! # Quick Start
//! The following example covers the primary functionalities of this library
//! and should be quick on understanding how to use the library.
//!
//! ## Async version
//! ```rust
//! use msq::{MSQClient, Region, Filter};
//! use std::io::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Startup the client
//!     let mut client = MSQClient::new().await?;
//!
//!     // Connect to the master server
//!     client.connect("hl2master.steampowered.com:27011").await?;
//!
//!     // Maximum amount of servers we wanted to query
//!     client.max_servers_on_query(256);
//!
//!     let servers = client
//!         .query(Region::Europe,  // Restrict query to Europe region
//!             Filter::new()       // Create a Filter builder
//!                 .appid(240)     // appid of 240 (CS:S)
//!                 .nand()         // Start of NAND special filter
//!                     .map("de_dust2")     // Map is de_dust2
//!                     .empty(true)         // Server is empty
//!                 .end()          // End of NAND special filter
//!                 .gametype(&vec!["friendlyfire", "alltalk"])).await?;
//!
//!     // nand filter excludes servers that has de_dust2 as
//!     // its map and is empty
//!
//!     // nand and nor are both special filters, both closed by
//!     // using the end method
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Blocking/Non-Async version
//! If you don't want to use async, then a blocking version is available.
//! The methods functionalities and names should matches its async
//! counterpart.
//! ```rust
//! use msq::{MSQClientBlock, Region, Filter};
//! use std::io::Result;
//!
//! fn main() -> Result<()> {
//!     let mut client = MSQClientBlock::new()?;
//!     client.connect("hl2master.steampowered.com:27011")?;
//!     client.max_servers_on_query(256);
//!
//!     let servers = client
//!         .query(Region::Europe,  // Restrict query to Europe region
//!             Filter::new()       // Create a Filter builder
//!                 .appid(240)     // appid of 240 (CS:S)
//!                 .nand()         // Start of NAND special filter
//!                     .map("de_dust2")     // Map is de_dust2
//!                     .empty(true)         // Server is empty
//!                 .end()          // End of NAND special filter
//!                 .gametype(&vec!["friendlyfire", "alltalk"]))?;
//!     Ok(())
//! }
//! ```

mod filter;
mod region;
mod packet_ext;

#[cfg(feature = "async")]
mod client_async;

#[cfg(feature = "non-async")]
mod client_blocking;

pub use crate::filter::Filter;
pub use crate::region::Region;

#[cfg(feature = "async")]
pub use crate::client_async::MSQClient;

#[cfg(feature = "non-async")]
pub use crate::client_blocking::MSQClientBlock;

