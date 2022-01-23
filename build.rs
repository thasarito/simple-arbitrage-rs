// use ethers_solc::{Project, ProjectPathsConfig};
use ethers::contract::MultiAbigen;

fn main() {
    // configure the project with all its paths, solc, cache etc.
    println!("Building ethers contract");
    let gen = MultiAbigen::from_json_files("./out").unwrap();
    gen.write_to_module("./src/bindings").unwrap();
    // new(vec![("IUniswapV2Pair", "./out/IUniswapV2Pair.sol/IUniswapV2Pair.json")]).unwrap().write_to_module("./src/bindings");
    println!("cargo:rerun-if-changed=src/contracts");
}
