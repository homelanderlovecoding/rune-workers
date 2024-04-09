use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use anyhow::{anyhow, bail, Context, ensure, Error};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::bitcoin::Network;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use crate::options::Options;
use crate::Result;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields)]
pub struct Settings {
  bitcoin_data_dir: Option<PathBuf>,
  bitcoin_rpc_password: Option<String>,
  bitcoin_rpc_url: Option<String>,
  bitcoin_rpc_username: Option<String>,
  bitcoin_last_block: Option<u32>,
  cookie_file: Option<PathBuf>,
}

impl Settings {
  // init bitcoin client here
  pub(crate) fn bitcoin_rpc_client(&mut self, wallet: Option<String>) -> Result<Client> {
    let rpc_url = self.bitcoin_rpc_url(wallet);

    let bitcoin_credentials = self.bitcoin_credentials()?;

    log::info!(
      "Connecting to Bitcoin Core at {}",
      self.bitcoin_rpc_url(None)
    );

    if let Auth::CookieFile(cookie_file) = &bitcoin_credentials {
      log::info!(
        "Using credentials from cookie file at `{}`",
        cookie_file.display()
      );

      ensure!(
        cookie_file.is_file(),
        "cookie file `{}` does not exist",
        cookie_file.display()
      );
    }

    let client = Client::new(&rpc_url, bitcoin_credentials)
        .with_context(|| format!("failed to connect to Bitcoin Core RPC at `{rpc_url}`"))?;

    Ok(client)
  }

  pub(crate) fn bitcoin_rpc_url(&self, wallet_name: Option<String>) -> String {
    let base_url = self.bitcoin_rpc_url.as_ref().unwrap();
    match wallet_name {
      Some(wallet_name) => format!("{base_url}/wallet/{wallet_name}"),
      None => format!("{base_url}/"),
    }
  }

  pub(crate) fn bitcoin_credentials(&self) -> Result<Auth> {
    if let Some((user, pass)) = &self
        .bitcoin_rpc_username
        .as_ref()
        .zip(self.bitcoin_rpc_password.as_ref())
    {
      Ok(Auth::UserPass((*user).clone(), (*pass).clone()))
    } else {
      Ok(Auth::CookieFile(self.cookie_file()?))
    }
  }

  pub(crate) fn cookie_file(&self) -> Result<PathBuf> {
    if let Some(cookie_file) = &self.cookie_file {
      return Ok(cookie_file.clone());
    }

    let path = if let Some(bitcoin_data_dir) = &self.bitcoin_data_dir {
      bitcoin_data_dir.clone()
    } else if cfg!(target_os = "linux") {
      dirs::home_dir()
          .ok_or_else(|| anyhow!("failed to get cookie file path: could not get home dir"))?
          .join(".bitcoin")
    } else {
      dirs::data_dir()
          .ok_or_else(|| anyhow!("failed to get cookie file path: could not get data dir"))?
          .join("Bitcoin")
    };

    let path = self.chain().join_with_data_dir(path);

    Ok(path.join(".cookie"))
  }


  pub(crate) fn from_options(options: Options) -> Self {
    Self {
      bitcoin_data_dir: options.bitcoin_data_dir,
      bitcoin_rpc_password: options.bitcoin_rpc_password,
      bitcoin_rpc_url: options.bitcoin_rpc_url,
      bitcoin_rpc_username: options.bitcoin_rpc_username,
      bitcoin_last_block: None,
      cookie_file: None,
    }
  }

  fn chain(&self) -> Chain {
    Chain::Regtest
  }
}

#[derive(Default, ValueEnum, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Chain {
  #[default]
  #[value(alias("main"))]
  Mainnet,
  #[value(alias("test"))]
  Testnet,
  Signet,
  Regtest,
}

impl Chain {
  // pub(crate) fn network(self) -> Network {
  //   self.into()
  // }

  pub(crate) fn default_rpc_port(self) -> u16 {
    match self {
      Self::Mainnet => 8332,
      Self::Regtest => 18443,
      Self::Signet => 38332,
      Self::Testnet => 18332,
    }
  }

  pub(crate) fn inscription_content_size_limit(self) -> Option<usize> {
    match self {
      Self::Mainnet | Self::Regtest => None,
      Self::Testnet | Self::Signet => Some(1024),
    }
  }

  pub(crate) fn first_inscription_height(self) -> u32 {
    match self {
      Self::Mainnet => 767430,
      Self::Regtest => 0,
      Self::Signet => 112402,
      Self::Testnet => 2413343,
    }
  }

  pub(crate) fn jubilee_height(self) -> u32 {
    match self {
      Self::Mainnet => 824544,
      Self::Regtest => 110,
      Self::Signet => 175392,
      Self::Testnet => 2544192,
    }
  }
  pub(crate) fn join_with_data_dir(self, data_dir: impl AsRef<Path>) -> PathBuf {
    match self {
      Self::Mainnet => data_dir.as_ref().to_owned(),
      Self::Testnet => data_dir.as_ref().join("testnet3"),
      Self::Signet => data_dir.as_ref().join("signet"),
      Self::Regtest => data_dir.as_ref().join("regtest"),
    }
  }
}
