use bitcoincore_rpc::bitcoin::Block;
use bitcoincore_rpc::{Client, RpcApi};
use crate::Result;

pub(crate) trait BitcoinCoreRpcResultExt<T> {
    fn into_option(self) -> Result<Option<T>>;
}

impl<T> BitcoinCoreRpcResultExt<T> for Result<T, bitcoincore_rpc::Error> {
    fn into_option(self) -> Result<Option<T>> {
        match self {
            Ok(ok) => Ok(Some(ok)),
            Err(bitcoincore_rpc::Error::JsonRpc(bitcoincore_rpc::jsonrpc::error::Error::Rpc(
                                                    bitcoincore_rpc::jsonrpc::error::RpcError { code: -8, .. },
                                                ))) => Ok(None),
            Err(bitcoincore_rpc::Error::JsonRpc(bitcoincore_rpc::jsonrpc::error::Error::Rpc(
                                                    bitcoincore_rpc::jsonrpc::error::RpcError { message, .. },
                                                )))
            if message.ends_with("not found") =>
                {
                    Ok(None)
                }
            Err(err) => Err(err.into()),
        }
    }
}

pub struct Index {
    pub(crate) client: Client,
}

impl Index {
    pub(crate) fn get_block_by_height(self, height: u32) -> Result<Option<Block>> {
        Ok(
            self
                .client
                .get_block_hash(height.into())
                .into_option()?
                .map(|hash| self.client.get_block(&hash))
                .transpose()?,
        )
    }
}
