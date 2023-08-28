use ethers::{
    abi::Contract,
    core::{k256::ecdsa::SigningKey, types::Bytes},
    solc::{CompilerInput, CompilerOutput, EvmVersion, Solc},
};
use ethers_contract_abigen::Abigen;
use glob::{glob, Paths, PatternError};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap, env, fs::File, path::Path, sync::Arc, thread::sleep, time::Duration, io::BufReader,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompiledContract {
    /// Contract path
    pub path: String,
    /// Contract name
    pub name: String,
    /// ABI
    pub abi: Contract,
    /// Bytecode
    pub bin: Bytes,
    /// Runtime Bytecode
    pub bin_runtime: Bytes,
}

/// Path to the test contracts
const CONTRACTS_PATH: &str = "contracts";
/// Solidity compilation warnings to ignore (by error code)
/// 2018: Warning - "Function state mutability can be restricted to pure"
/// 5667: Warning - "Unused function parameter. Remove or comment out the
/// variable name to silence this warning."
/// For smart contracts that are optimized for worst case block generation, we want to allow
/// contracts that do not interfere with state, without setting state mutability to view. otherwise
/// compiler optimizations will not allow recursive execution of targeted opcodes
const WARN: &[u64] = &[2018, 5667];
/// List of contracts as (ContractName, ContractSolidityFile)
const CONTRACTS: &[(&str, &str)] = &[
    ("Greeter", "Greeter.sol"),
    (
        "OpenZeppelinERC20TestToken",
        "OpenZeppelinERC20TestToken.sol",
    ),
    // Contract to test worst-case usage of opcodes.
    ("Benchmarks", "BENCHMARKS.sol"),
];

// fn read_contracts_from_file<P: AsRef<Path>>(path: P) -> Result<, Box<dyn Error>> {

// }

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let contracts_dir = "./contracts/";
    let bindings_dir = "src";
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let solc = Solc::default();
    info!("Solc version {}", solc.version().expect("version works"));
    let mut contracts = HashMap::new();
    for (name, contract_path) in CONTRACTS {
        let path_sol = Path::new(CONTRACTS_PATH).join(contract_path);
        let inputs = CompilerInput::new(&path_sol).expect("Compile success");
        // ethers-solc: explicitly indicate the EvmVersion that corresponds to the zkevm circuit's
        // supported Upgrade, e.g. `London/Shanghai/...` specifications.
        let input = inputs
            .clone()
            .first_mut()
            .expect("first exists")
            .clone()
            .evm_version(EvmVersion::London);

        // compilation will either fail with Err variant or return Ok(CompilerOutput)
        // which may contain Errors or Warnings
        let output = solc.compile_output(&input).unwrap();
        let mut deserializer: serde_json::Deserializer<serde_json::de::SliceRead<'_>> =
            serde_json::Deserializer::from_slice(&output);
        // The contracts to test the worst-case usage of certain opcodes, such as SDIV, MLOAD, and
        // EXTCODESIZE, generate big JSON compilation outputs. We disable the recursion limit to
        // avoid parsing failure.
        deserializer.disable_recursion_limit();
        let compiled = match CompilerOutput::deserialize(&mut deserializer) {
            Err(error) => {
                panic!("COMPILATION ERROR {:?}\n{:?}", &path_sol, error);
            }
            // CompilationOutput is succesfully created (might contain Errors or Warnings)
            Ok(output) => {
                info!("COMPILATION OK: {:?}", name);
                output
            }
        };

        if compiled.has_error() || compiled.has_warning(WARN) {
            panic!(
                "... but CompilerOutput contains errors/warnings: {:?}:\n{:#?}",
                &path_sol, compiled.errors
            );
        }

        let contract = compiled
            .get(path_sol.to_str().expect("path is not str"), name)
            .expect("contract not found");
        let abi = contract.abi.expect("no abi found").clone();
        let bin = contract.bin.expect("no bin found").clone();
        let bin_runtime = contract.bin_runtime.expect("no bin_runtime found").clone();
        let compiled_contract = CompiledContract {
            path: path_sol.to_str().expect("path is not str").to_string(),
            name: name.to_string(),
            abi,
            bin: bin.into_bytes().expect("bin"),
            bin_runtime: bin_runtime.into_bytes().expect("bin_runtime"),
        };

        let mut path_json = path_sol.clone();
        path_json.set_extension("json");
        serde_json::to_writer(
            &File::create(&path_json).expect("cannot create file"),
            &compiled_contract,
        )
        .expect("cannot serialize json into file");

        contracts.insert(name.to_string(), compiled_contract);
    }
    eprintln!("DUMPING COMPILED CONTRACTS");
    // eprintln!("CONTRACTS: {:#?}", contracts);
    let mut path_json = Path::new(CONTRACTS_PATH)
        .join("compiled_contracts")
        .join("compiled_contracts")
        .clone();
    path_json.set_extension("json");
    serde_json::to_writer(
        &File::create(&path_json).expect("cannot create file"),
        &contracts,
    )
    .expect("cannot serialize json into file");

    let _ = uneval::to_file(contracts, Path::new(bindings_dir).join("contracts.rs"))
        .expect("Write failed");

    eprintln!("COMPILED CONTRACTS DUMPED SUCCESFULLY");
    for entry in glob::glob("./contracts/*.json").unwrap() {
        match entry {
            Ok(path) => {
                let abi_source = path.clone();
                let tempbinding = path
                    .into_os_string()
                    .into_string()
                    .expect("FAILED CONVERSION TO STRING");
                let filenamevec: Vec<&str> = tempbinding.split("/").collect();
                let filename = filenamevec[1];

                let contractnamevector: Vec<&str> = filename.split(".").collect();
                let contractname = contractnamevector[0].to_lowercase();
                let destpath = Path::new(&bindings_dir)
                    // .join(&bindings_dir)
                    .join([contractname.clone(), "rs".to_string()].join("."));
                eprintln!("{:#?}", destpath);
                let _ = Abigen::new(
                    contractname,
                    abi_source.into_os_string().into_string().expect("REASON"),
                )
                // .unwrap()
                .expect("error")
                .generate()
                .expect("error")
                .write_to_file(destpath);
            }
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    let con = load_compiled_contracts("contracts/compiled_contracts/compiled_contracts.json");
    eprintln!("CON: {:#?}", con);
    panic!();
}



fn load_compiled_contracts(path: &str) -> () { //HashMap<&str, CompiledContract> {
    let file = std::fs::File::open(path).expect("error reading file");
    let reader = std::io::BufReader::new(file);
    // let preader: HashMap<&str,_> = serde_json::from_reader::<BufReader<std::fs::File>, HashMap<&str, _>>(reader).expect("REASON");
    // eprintln!("READER: {:#?}", preader);
    // let contracts: HashMap<String, CompiledContract> = HashMap::new();

    let jjson: String = serde_json::from_reader(reader).expect("error");
    eprintln!("JJSON: {:#?}", jjson.to_string());
    ()
}