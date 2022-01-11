use std::sync::Arc;
use std::time::Duration;

use ethers::prelude::*;
use ethers::utils::GanacheInstance;

use ethers::core::{abi::Abi, types::Bytes};
use ethers::providers::Http;

// connects the private key to http://localhost:8545
pub fn connect(ganache: &GanacheInstance, idx: usize) -> Arc<Provider<Http>> {
    let sender = ganache.addresses()[idx];
    let provider = Provider::<Http>::try_from(ganache.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64))
        .with_sender(sender);
    Arc::new(provider)
}

/// compiles the given contract and returns the ABI and Bytecode
pub fn compile_contract(name: &str, filename: &str) -> (Abi, Bytes) {
    let path = format!("./src/contracts/{}", filename);
    dbg!(&path);
    let compiler = Solc::default(); //.arg("--evm-version").arg("petersburg");
    dbg!(&compiler);
    // compiler.arg()
    let compiled = compiler.compile_source(&path).unwrap();
    dbg!(&compiled);
    let contract = compiled.get(&path, name).expect("could not find contract");
    let (abi, bin, _) = contract.into_parts_or_default();
    (abi, bin)
}
