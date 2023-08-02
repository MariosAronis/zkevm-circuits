//! Integration testing

#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]

use bus_mapping::rpc::GethClient;
use env_logger::Env;
use eth_types::Address;
use ethers::{
    abi,
    core::{k256::ecdsa::SigningKey, types::Bytes},
    providers::{Http, Provider},
    signers::{coins_bip39::English, MnemonicBuilder, Signer, Wallet},
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::{self, VarError},
    fs::File,
    sync::Once,
    time::Duration,
};
use url::Url;

/// Geth dev chain ID
pub const CHAIN_ID: u64 = 1337;
/// Path to the test contracts
pub const CONTRACTS_PATH: &str = "contracts";
/// Solidity compilation warnings to ignore (by error code)
pub const WARN: &[u64] = &[];
/// List of contracts as (ContractName, ContractSolidityFile)
pub const CONTRACTS: &[(&str, &str)] = &[
    ("Greeter", "greeter/Greeter.sol"),
    (
        "OpenZeppelinERC20TestToken",
        "ERC20/OpenZeppelinERC20TestToken.sol",
    ),
    ("CheckMload", "MLOAD/MLOAD.sol"),
    ("CheckExtCodeSize100", "EXTCODESIZE/EXTCODESIZE100.sol"),
    ("CheckSdiv", "SDIV/SDIV.sol"),
];
/// Path to gen_blockchain_data output file
pub const GENDATA_OUTPUT_PATH: &str = "gendata_output.json";

const GETH0_URL_DEFAULT: &str = "http://localhost:8545";

lazy_static! {
    /// URL of the integration test geth0 instance, which contains blocks for which proofs will be
    /// generated.
    pub static ref GETH0_URL: String = match env::var("GETH0_URL") {
        Ok(val) => val,
        Err(VarError::NotPresent) => GETH0_URL_DEFAULT.to_string(),
        Err(e) => panic!("Error in GETH0_URL env var: {:?}", e),
    };
}

static LOG_INIT: Once = Once::new();

/// Initialize log
pub fn log_init() {
    LOG_INIT.call_once(|| {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    });
}

/// Get the integration test [`GethClient`]
pub fn get_client() -> GethClient<Http> {
    let transport = Http::new(Url::parse(&GETH0_URL).expect("invalid url"));
    GethClient::new(transport)
}

/// Get the integration test [`Provider`]
pub fn get_provider() -> Provider<Http> {
    let transport = Http::new(Url::parse(&GETH0_URL).expect("invalid url"));
    Provider::new(transport).interval(Duration::from_millis(100))
}

/// Get the chain id by querying the geth client.
pub async fn get_chain_id() -> u64 {
    let client = get_client();
    client.get_chain_id().await.unwrap()
}

const PHRASE: &str =
    "work man father plunge mystery proud hollow address reunion sauce theory bonus";

/// Get a wallet by index
pub fn get_wallet(index: u32) -> Wallet<SigningKey> {
    // Access mnemonic phrase.
    // Child key at derivation path: m/44'/60'/0'/0/{index}
    MnemonicBuilder::<English>::default()
        .phrase(PHRASE)
        .index(index)
        .expect("invalid index")
        .build()
        .expect("cannot build wallet from mnemonic")
        .with_chain_id(CHAIN_ID)
}

/// Output information of the blockchain data generated by
/// `gen_blockchain_data`.
#[derive(Serialize, Deserialize)]
pub struct GenDataOutput {
    /// Coinbase of the blockchain
    pub coinbase: Address,
    /// Wallets used by `gen_blockchain_data`
    pub wallets: Vec<Address>,
    /// Block map: BlockContent -> BlockNum
    pub blocks: HashMap<String, u64>,
    /// Contracts deployed map: ContractName -> (BlockNum, Address)
    pub deployments: HashMap<String, (u64, Address)>,
}

impl GenDataOutput {
    /// Load [`GenDataOutput`] from the json file.
    pub fn load() -> Self {
        serde_json::from_reader(File::open(GENDATA_OUTPUT_PATH).expect("cannot read file"))
            .expect("cannot deserialize json from file")
    }

    /// Store [`GenDataOutput`] into the json file.
    pub fn store(&self) {
        serde_json::to_writer(
            &File::create(GENDATA_OUTPUT_PATH).expect("cannot create file"),
            self,
        )
        .expect("cannot serialize json into file");
    }
}

/// Solc-compiled contract output
#[derive(Serialize, Deserialize)]
pub struct CompiledContract {
    /// Contract path
    pub path: String,
    /// Contract name
    pub name: String,
    /// ABI
    pub abi: abi::Contract,
    /// Bytecode
    pub bin: Bytes,
    /// Runtime Bytecode
    pub bin_runtime: Bytes,
}

/// Common code for integration tests of circuits.
pub mod integration_test_circuits;
