use bitcoind;
#[cfg(feature = "download")]
#[cfg(feature = "26_1")]
fn main() {
    use bitcoincore_rpc::RpcApi;
    let bitcoind = bitcoind::BitcoinD::from_downloaded().unwrap();
    println!("{}",bitcoind.client.get_blockchain_info().unwrap().blocks);
    println!("{}",bitcoind.client.get_blockchain_info().unwrap().blocks);
    //assert_eq!(0, bitcoind.client.get_blockchain_info().unwrap().blocks);
    println!("{:?}",bitcoind::downloaded_exe_path().unwrap());

    let mut conf = bitcoind::Conf::default();
    //conf.args = vec!["-regtest", "-fallbackfee=0.0001"];
    //conf.view_stdout = false;
    //conf.p2p = bitcoind::P2P::No;
    //conf.network = "regtest";
    //conf.tmpdir = None;
    //conf.staticdir = None;
    conf.attempts = 3;
    //assert_eq!(conf, bitcoind::Conf::default());

    conf.args = vec!["-signet", "-signetchallenge=512102ee856c56a5aaadd1656f849bafa4c9dacc86a2878fe546c6189185f842ae2c1851ae"];
    conf.view_stdout = true;
    println!("{}",bitcoind.client.get_blockchain_info().unwrap().blocks);
    println!("{}",bitcoind.client.get_blockchain_info().unwrap().blocks);
    conf.p2p = bitcoind::P2P::Yes;
    println!("{:?}",bitcoind::downloaded_exe_path().unwrap());
}
