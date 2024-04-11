use bitcoind;
fn main() {

// the download feature is enabled whenever a specific version is enabled, for example `25_1` or `24_0_1`
//#[cfg(feature = "download")]
//{
  use bitcoincore_rpc::RpcApi;
  let bitcoind = bitcoind::BitcoinD::from_downloaded().unwrap();
  assert_eq!(0, bitcoind.client.get_blockchain_info().unwrap().blocks);
//}

}
