use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Error;
use bitcoincore_rpc::bitcoin::Transaction;
use bitcoincore_rpc::RpcApi;
use clap::Parser;
use ord::RuneEntry;
use ordinals::{RuneId, Runestone};
use ordinals::Artifact;
use crate::lot::Lot;

use crate::options::Options;
use crate::rune_index::Index;
use crate::settings::Settings;

mod settings;
mod options;
mod rune_index;
mod lot;

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() {
    println!("Hello, world!");

    // test connection
    let mut settings = Settings::from_options(
        Options::try_parse_from([
            "ord",
            "--bitcoin-data-dir=/Volumes/Suong/canlab/ord/env",
            // "--bitcoin-rpc-password=bitcoin password",
            "--bitcoin-rpc-url=http://127.0.0.1:9000",
            // "--bitcoin-rpc-username=bitcoin username",
        ])
        .unwrap()
    );

    let client  = settings.bitcoin_rpc_client(None).unwrap();

    let index = Index {
        client,
    };

    let starting_height = u32::try_from(index.client.get_block_count().unwrap()).unwrap();
    /// print starting height
    println!("{}", starting_height);

    let block_hash = index.client.get_block_hash(starting_height as u64).unwrap();
    println!("{}", block_hash);

    // get block by hash
    let block = index.get_block_by_height(starting_height).unwrap().unwrap();
    println!("{:?}", block);


    let mut id_to_entry: HashMap<&RuneId, RuneEntry> =
        [].iter().cloned().collect();

    /// loop thru tx in block
    for tx in block.txdata {
        /// detect new etch
        let artifact = Runestone::decipher(&tx);
        let mut unallocated = unallocated(&tx)?;

        if let Some(artifact) = &artifact {
            if let Some(id) = artifact.mint() {
                if let Some(amount) = self.mint(id)? {
                    *unallocated.entry(id).or_default() += amount;
                }
            }

            let etched = self.etched(tx_index, tx, artifact)?;

            if let Artifact::Runestone(runestone) = artifact {
                if let Some((id, ..)) = etched {
                    *unallocated.entry(id).or_default() +=
                        runestone.etching.unwrap().premine.unwrap_or_default();
                }
            }

            if let Some((id, rune)) = etched {
                self.create_rune_entry(txid, artifact, id, rune)?;
            }
        }
    }


    fn unallocated(tx: &Transaction) -> Result<HashMap<RuneId, Lot>> {
        // map of rune ID to un-allocated balance of that rune
        let mut unallocated: HashMap<RuneId, Lot> = HashMap::new();

        Ok(unallocated)
    }


    fn mint(id: RuneId) -> Result<Option<Lot>> {
        let Some(entry) = self.id_to_entry.get(&id.store())? else {
            return Ok(None);
        };

        let mut rune_entry = RuneEntry::load(entry.value());

        let Ok(amount) = rune_entry.mintable(self.height.into()) else {
            return Ok(None);
        };

        drop(entry);

        rune_entry.mints += 1;

        id_to_entry.insert(&id.store(), rune_entry.store())?;

        Ok(Some(Lot(amount)))
    }

}
