use std::path::PathBuf;
use bitcoincore_rpc::RpcApi;
use clap::Parser;
use crate::options::Options;
use crate::settings::Settings;

mod settings;
mod options;

fn main() {
    println!("Hello, world!");

    // test connection
    let settings = Settings::from_options(
        Options::try_parse_from([
            "ord",
            "--bitcoin-data-dir=/Users/0xhomelander/Documents/ord/env",
            // "--bitcoin-rpc-password=bitcoin password",
            "--bitcoin-rpc-url=http://127.0.0.1:9000",
            // "--bitcoin-rpc-username=bitcoin username",
        ])
        .unwrap()
    );

    let client = settings.bitcoin_rpc_client(None)?;
    let starting_height = u32::try_from(client.get_block_count()?).unwrap() + 1;
    /// print starting height
    println!("{}", starting_height)

    // get block info

}
