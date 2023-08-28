use ethers::{
    abi::{self, Detokenize, Tokenize, Function},
    contract::{builders::ContractCall, Contract, ContractFactory, ContractInstance},
    core::{
        types::{
            transaction::eip2718::TypedTransaction, Address, GethDebugTracingOptions, GethTrace,
            TransactionReceipt, TransactionRequest, U256, U64,
        },
        utils::WEI_IN_ETHER,
    },
    middleware::SignerMiddleware,
    providers::{Middleware, PendingTransaction},
    signers::Signer,
    solc::{CompilerInput, CompilerOutput, EvmVersion, Solc},
    types::{GethDebugTracerConfig, GethDebugTracerType},
};
use ethers_contract_abigen::{Abigen, MultiAbigen};
use glob::glob;
// use integration_tests::mload;
use integration_tests::{
    get_client,
    get_provider,
    get_wallet,
    log_init,
    CompiledContract,
    GenDataOutput,
    CONTRACTS,
    CONTRACTS_PATH,
    WARN, // contract_bindings::*,
};
use log::{error, info};
use serde::de::Deserialize;
use std::{collections::HashMap, fs::File, path::Path, sync::Arc, thread::sleep, time::Duration};

async fn deploy<T, M>(prov: Arc<M>, compiled: &CompiledContract, args: T) -> Contract<M>
where
    T: Tokenize,
    M: Middleware,
{
    info!("Deploying {}...", compiled.name);
    let factory = ContractFactory::new(compiled.abi.clone(), compiled.bin.clone(), prov);
    factory
        .deploy(args)
        .expect("cannot deploy")
        .confirmations(0usize)
        .send()
        .await
        .expect("cannot confirm deploy")
}

fn erc20_transfer<M>(
    prov: Arc<M>,
    contract_address: Address,
    contract_abi: &abi::Contract,
    to: Address,
    amount: U256,
) -> TypedTransaction
where
    M: Middleware,
{
    let contract = Contract::new(contract_address, contract_abi.clone(), prov);
    let call: ContractCall<M, _> = contract
        .method::<_, bool>("transfer", (to, amount))
        .expect("cannot construct ERC20 transfer call");
    // Set gas to avoid `eth_estimateGas` call
    let call = call.legacy();
    let call = call.gas(100_000);
    call.tx
}

async fn send_confirm_tx<M>(prov: &Arc<M>, tx: TypedTransaction) -> TransactionReceipt
where
    M: Middleware,
{
    prov.send_transaction(tx, None)
        .await
        .expect("cannot send ERC20 transfer call")
        .confirmations(0usize)
        .await
        .unwrap()
        .unwrap()
}

#[tokio::main]
async fn main() {
    log_init();

    

    // Read the JSON contents of the file as an instance of `Person`.
    let contracts: HashMap<&str, &str> = serde_json::from_reader(reader).expect("REASON");
    // let contracts = in   clude!(concat!("../", "contracts.rs"));

    // Compile contracts
    // info!("Compiling contracts...");
    // let solc = Solc::default();
    // info!("Solc version {}", solc.version().expect("version works"));
    // let mut contracts = HashMap::new();
    // for (name, contract_path) in CONTRACTS {
    // let path_sol = Path::new(CONTRACTS_PATH).join(contract_path);
    // let inputs = CompilerInput::new(&path_sol).expect("Compile success");
    // ethers-solc: explicitly indicate the EvmVersion that corresponds to the zkevm circuit's
    // supported Upgrade, e.g. `London/Shanghai/...` specifications.
    // let input = inputs
    // .clone()
    // .first_mut()
    // .expect("first exists")
    // .clone()
    // .evm_version(EvmVersion::London);

    // compilation will either fail with Err variant or return Ok(CompilerOutput)
    // which may contain Errors or Warnings
    // let output = solc.compile_output(&input).unwrap();
    // let mut deserializer: serde_json::Deserializer<serde_json::de::SliceRead<'_>> =
    // serde_json::Deserializer::from_slice(&output);
    // The contracts to test the worst-case usage of certain opcodes, such as SDIV, MLOAD, and
    // EXTCODESIZE, generate big JSON compilation outputs. We disable the recursion limit to
    // avoid parsing failure.
    // deserializer.disable_recursion_limit();
    // let compiled = match CompilerOutput::deserialize(&mut deserializer) {
    // Err(error) => {
    // panic!("COMPILATION ERROR {:?}\n{:?}", &path_sol, error);
    // }
    // CompilationOutput is succesfully created (might contain Errors or Warnings)
    // Ok(output) => {
    // info!("COMPILATION OK: {:?}", name);
    // output
    // }
    // };

    // if compiled.has_error() || compiled.has_warning(WARN) {
    // panic!(
    // "... but CompilerOutput contains errors/warnings: {:?}:\n{:#?}",
    // &path_sol, compiled.errors
    // );
    // }

    // let contract = compiled
    // .get(path_sol.to_str().expect("path is not str"), name)
    // .expect("contract not found");
    // let abi = contract.abi.expect("no abi found").clone();
    // let bin = contract.bin.expect("no bin found").clone();
    // let bin_runtime = contract.bin_runtime.expect("no bin_runtime found").clone();
    // let compiled_contract = CompiledContract {
    // path: path_sol.to_str().expect("path is not str").to_string(),
    // name: name.to_string(),
    // abi,
    // bin: bin.into_bytes().expect("bin"),
    // bin_runtime: bin_runtime.into_bytes().expect("bin_runtime"),
    // };

    // let mut path_json = path_sol.clone();
    // path_json.set_extension("json");
    // serde_json::to_writer(
    // &File::create(&path_json).expect("cannot create file"),
    // &compiled_contract,
    // )
    // .expect("cannot serialize json into file");

    // contracts.insert(name.to_string(), compiled_contract);
    // }

    let prov = get_provider();

    // Wait for geth to be online.
    loop {
        match prov.client_version().await {
            Ok(version) => {
                info!("Geth online: {}", version);
                break;
            }
            Err(err) => {
                error!("Geth not available: {:?}", err);
                sleep(Duration::from_millis(500));
            }
        }
    }

    // Make sure the blockchain is in a clean state: block 0 is the last block.
    let block_number = prov
        .get_block_number()
        .await
        .expect("cannot get block number");
    if block_number.as_u64() != 0 {
        panic!(
            "Blockchain is not in a clean state.  Last block number: {}",
            block_number
        );
    }

    let accounts = prov.get_accounts().await.expect("cannot get accounts");
    let wallet0 = get_wallet(0);
    info!("wallet0: {:x}", wallet0.address());

    let mut blocks = HashMap::new();

    // ETH Transfer: Transfer funds to our account.
    //

    info!("Transferring funds from coinbase...");
    let tx = TransactionRequest::new()
        .to(wallet0.address())
        .value(WEI_IN_ETHER) // send 1 ETH
        .from(accounts[0]);
    prov.send_transaction(tx, None)
        .await
        .expect("cannot send tx")
        .await
        .expect("cannot confirm tx");
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Transfer 0".to_string(), block_num.as_u64());

    // Deploy smart contracts
    //
    let mut contract_apis = HashMap::new();
    let mut deployments = HashMap::new();
    let prov_wallet0 = Arc::new(SignerMiddleware::new(get_provider(), wallet0));

    // Greeter
    let contract = deploy(
        prov_wallet0.clone(),
        contracts.get("Greeter").expect("contract not found"),
        U256::from(42),
    )
    .await;
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Deploy Greeter".to_string(), block_num.as_u64());
    deployments.insert(
        "Greeter".to_string(),
        (block_num.as_u64(), contract.address()),
    );
    contract_apis.insert("Greeter".to_string(), contract);

    // OpenZeppelinERC20TestToken
    let contract = deploy(
        prov_wallet0.clone(),
        contracts
            .get("OpenZeppelinERC20TestToken")
            .expect("contract not found"),
        prov_wallet0.address(),
    )
    .await;
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert(
        "Deploy OpenZeppelinERC20TestToken".to_string(),
        block_num.as_u64(),
    );
    deployments.insert(
        "OpenZeppelinERC20TestToken".to_string(),
        (block_num.as_u64(), contract.address()),
    );
    contract_apis.insert("OpenZeppelinERC20TestToken".to_string(), contract);

    // Deploy smart contracts for worst case block benches
    //

    // CheckMload
    let contract = deploy(
        prov_wallet0.clone(),
        contracts.get("CheckMload").expect("contract not found"),
        (),
    )
    .await;
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Deploy CheckMload".to_string(), block_num.as_u64());
    deployments.insert(
        "CheckMload".to_string(),
        (block_num.as_u64(), contract.address()),
    );
    contract_apis.insert("CheckMload".to_string(), contract);

    // CheckSdiv
    let contract = deploy(
        prov_wallet0.clone(),
        contracts.get("CheckSdiv").expect("contract not found"),
        (),
    )
    .await;
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Deploy CheckSdiv".to_string(), block_num.as_u64());
    deployments.insert(
        "CheckSdiv".to_string(),
        (block_num.as_u64(), contract.address()),
    );
    contract_apis.insert("CheckSdiv".to_string(), contract);

    // CheckExtCodeSize
    let contract = deploy(
        prov_wallet0.clone(),
        contracts
            .get("CheckExtCodeSize")
            .expect("contract not found"),
        (),
    )
    .await;
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Deploy CheckExtCodeSize".to_string(), block_num.as_u64());
    deployments.insert(
        "CheckExtCodeSize".to_string(),
        (block_num.as_u64(), contract.address()),
    );
    contract_apis.insert("CheckExtCodeSize".to_string(), contract);

    // ETH transfers: Generate a block with multiple transfers
    //

    info!("Generating block with multiple transfers...");
    const NUM_TXS: usize = 4; // NUM_TXS must be >= 4 for the rest of the cases to work.
    let wallets: Vec<_> = (0..NUM_TXS + 1)
        .map(|i| Arc::new(SignerMiddleware::new(get_provider(), get_wallet(i as u32))))
        .collect();

    let cli = get_client();

    // Fund NUM_TXS wallets from coinbase
    cli.miner_stop().await.expect("cannot stop miner");
    let mut pending_txs = Vec::new();
    for wallet in &wallets[0..NUM_TXS] {
        let tx = TransactionRequest::new()
            .to(wallet.address())
            .value(WEI_IN_ETHER * 2u8) // send 2 ETH
            .from(accounts[0]);
        pending_txs.push(
            prov.send_transaction(tx, None)
                .await
                .expect("cannot send tx"),
        );
    }
    cli.miner_start().await.expect("cannot start miner");
    for tx in pending_txs {
        tx.await.expect("cannot confirm tx");
    }
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Fund wallets".to_string(), block_num.as_u64());

    // Make NUM_TXS transfers in a "chain"
    cli.miner_stop().await.expect("cannot stop miner");
    let mut pending_txs = Vec::new();
    for i in 0..NUM_TXS {
        let tx = TransactionRequest::new()
            .to(wallets[i + 1].address())
            .value(WEI_IN_ETHER / (2 * (i + 1))) // send a fraction of an ETH
            .from(wallets[i].address());
        pending_txs.push(
            wallets[i]
                .send_transaction(tx, None)
                .await
                .expect("cannot send tx"),
        );
    }
    cli.miner_start().await.expect("cannot start miner");
    for tx in pending_txs {
        tx.await.expect("cannot confirm tx");
    }
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert("Multiple transfers 0".to_string(), block_num.as_u64());

    // ERC20 calls (OpenZeppelin)
    //

    info!("Generating ERC20 calls...");

    let contract_address = deployments
        .get("OpenZeppelinERC20TestToken")
        .expect("contract not found")
        .1;
    let contract_abi = &contracts
        .get("OpenZeppelinERC20TestToken")
        .expect("contract not found")
        .abi;

    // OpenZeppelin ERC20 single failed transfer (wallet2 sends 345.67 Tokens to
    // wallet3, but wallet2 has 0 Tokens)
    info!("Doing OpenZeppelin ERC20 single failed transfer...");
    let amount = U256::from_dec_str("345670000000000000000").unwrap();
    let tx = erc20_transfer(
        wallets[2].clone(),
        contract_address,
        contract_abi,
        wallets[3].address(),
        amount,
    );
    let receipt = send_confirm_tx(&wallets[2], tx).await;
    assert_eq!(receipt.status, Some(U64::from(0u64)));
    blocks.insert(
        "ERC20 OpenZeppelin transfer failed".to_string(),
        receipt.block_number.unwrap().as_u64(),
    );

    // OpenZeppelin ERC20 single successful transfer (wallet0 sends 123.45 Tokens to
    // wallet4)
    info!("Doing OpenZeppelin ERC20 single successful transfer...");
    let amount = U256::from_dec_str("123450000000000000000").unwrap();
    let tx = erc20_transfer(
        wallets[0].clone(),
        contract_address,
        contract_abi,
        wallets[4].address(),
        amount,
    );
    let receipt = send_confirm_tx(&wallets[0], tx).await;
    assert_eq!(receipt.status, Some(U64::from(1u64)));
    blocks.insert(
        "ERC20 OpenZeppelin transfer successful".to_string(),
        receipt.block_number.unwrap().as_u64(),
    );

    // OpenZeppelin ERC20 multiple transfers in a single block (some successful,
    // some unsuccessful)
    // - wallet0 -> wallet1 (ok)
    // - wallet2 -> wallet3 (ko)
    // - wallet1 -> wallet0 (ok)
    // - wallet3 -> wallet2 (ko)
    info!("Doing OpenZeppelin ERC20 multiple transfers...");
    cli.miner_stop().await.expect("cannot stop miner");
    let mut tx_hashes = Vec::new();
    for (i, (from_i, to_i)) in [(0, 1), (2, 3), (1, 0), (3, 2)].iter().enumerate() {
        let amount = U256::from(0x800000000000000 / (i + 1));
        let prov_wallet = &wallets[*from_i];
        let tx = erc20_transfer(
            prov_wallet.clone(),
            contract_address,
            contract_abi,
            wallets[*to_i].address(),
            amount,
        );

        let pending_tx = prov_wallet
            .send_transaction(tx, None)
            .await
            .expect("cannot send ERC20 transfer call");
        let tx_hash = *pending_tx; // Deref for PendingTransaction returns TxHash
        tx_hashes.push(tx_hash);
    }
    cli.miner_start().await.expect("cannot start miner");
    for (i, tx_hash) in tx_hashes.iter().enumerate() {
        let pending_tx = PendingTransaction::new(*tx_hash, wallets[i].inner());
        let receipt = pending_tx.confirmations(0usize).await.unwrap().unwrap();
        let expected_status = if i % 2 == 0 { 1u64 } else { 0u64 };
        assert_eq!(
            receipt.status,
            Some(U64::from(expected_status)),
            "failed tx hash: {:?}, receipt: {:#?}",
            tx_hash,
            receipt
        );
    }
    let block_num = prov.get_block_number().await.expect("cannot get block_num");
    blocks.insert(
        "Multiple ERC20 OpenZeppelin transfers".to_string(),
        block_num.as_u64(),
    );

    let gen_data = GenDataOutput {
        coinbase: accounts[0],
        wallets: wallets.iter().map(|w| w.address()).collect(),
        blocks,
        deployments,
    };

    // let binding = c_instance
    //     .method::<_, String>("checkBatchYul", [100])
    //     .expect("REASON");

    // let meths = &binding;

    // pub fn habibi(name: String, abipath: String, jsonname: String) -> () {
    //     let _ = Abigen::new(name, abipath)
    //     .expect("error")
    //     .generate()
    //     .expect("error")
    //     .write_to_file(jsonname);
    // }

    // let binding1 = Abigen::from_file("contracts/MLOAD.json")
    //     .expect("REASON");
    // let binding2 = Abigen::from_file("contracts/SDIV.json")
    // let multia = MultiAbigen::from_json_files("./contracts")
    //     .expect("REASON");

    // #[derive(Clone,Debug, IntoIterator)]
    // pub struct JsonContract(String, String)

    // #[derive(Clone,Debug, IntoIterator)]
    // let jsonontracts: Vec<JsonContract> = [
    //     JsonContract("MLOAD".to_string(), "MLOAD.json".to_string()),
    //     JsonContract("SDIV".to_string(), "SDIV.json".to_string()),
    //     JsonContract("EXTCODESIZE".to_string(), "EXTCODESIZE.json".to_string()),
    //     JsonContract("Greeter".to_string(), "Greeter.json".to_string()),
    //     JsonContract(
    //         "OpenZeppelinERC20TestToken".to_string(),
    //         "OpenZeppelinERC20TestToken.json".to_string()
    //     ),
    // ].to_vec();

    // let jsoncontracts = [
    //     ("Greeter".to_string(), "Greeter.sol".to_string()),
    //     (
    //         "OpenZeppelinERC20TestToken".to_string(),
    //         "OpenZeppelinERC20TestToken.sol".to_string(),
    //     ),
    //     // Contracts to test worst-case usage of opcodes.
    //     ("CheckMload".to_string(), "MLOAD.sol".to_string()),
    //     ("CheckExtCodeSize".to_string(), "EXTCODESIZE.sol".to_string()),
    //     ("CheckSdiv".to_string(), "SDIV.sol".to_string()),
    // ];

    for entry in glob("contracts/*.json") {
        println!("{:#?}", entry);
    }

    // let js = jsoncontracts;//.to_vec();
    // info!("SOLCONTRACTS: {:#?}", js.IntoIterator());
    // info!("SOLCONTRACTS: {:#?}", js);

    // let multia = MultiAbigen::new(js.iter());
    // let binding1 = Abigen::from_file("c9ontracts/EXTCODESIZE.json")
    // .expect("REASON");
    // .unwrap();

    // let multia = match MultiAbigen::from_json_files("./contracts") {
    //     Err(error) => {
    //         panic!("error: {:#?}", error);
    //         // error
    //     }
    //     Ok(multiabigen) => {
    //                 info!("MULTIABIGEN SUCCESS");
    //                 multiabigen
    //             },
    // };

    // habibi("mload".to_string(),"contracts/MLOAD/MLOAD.json".to_string(), "mload.rs".to_string());
    // habibi("extcodesize".to_string(),"contracts/EXTCODESIZE/EXTCODESIZE.json".to_string(),
    // "extcodesize.rs".to_string());habibi("extcodesize".to_string(),"contracts/SDIV/SDIV.json".
    // to_string(), "sdiv.rs".to_string()); habibi("sdiv".to_string(),"contracts/SDIV/SDIV.json"
    // .to_string(), "sdiv.rs".to_string());

    // Abigen::new("CheckMload", "contracts/MLOAD/MLOAD.json")
    // .expect("error")
    // .generate()
    // .expect("ERROR")
    // .write_to_file("mload.rs");

    let block_num = prov.get_block_number().await.expect("cannot get block_num");

    info!("BLOCKNUMBER: {:#?}", block_num);
    // cli.miner_start().await.expect("cannot start miner");
    // let data = Len(10000);
    // let data2 = CheckBatchYulCall {l:data};
    // let cinstance = contract_apis
    //     .get("CheckMLoad")
    //     .expect("ERROR");

    // let binding = &cinstance
    //     .method::<mload::CheckBatchYulCall, String>("checkBatchYul", data2)
    //     .expect("ANOTHER ERROR");
    // let tx_builder = binding;

    // let response = tx_builder
    //     .send()
    //     .await
    //     .expect("REASON");
    // .unwrap();

    // check tx state until confirmed"

    // info!("RESPONSE: {:#?}", response);
    // cli.miner_stop().await.expect("cannot stop miner");
    // let block_num = prov.get_block_number().await.expect("cannot get block_num");
    // info! ("BLOCKNUMBER: {:#?}", block_num);

    // let opt = GethDebugTracingOptions::default();
    // let smtg = prov.debug_trace_transaction(response.tx_hash(), opt).await.unwrap();

    // match smtg {
    //     GethTrace::Known(a) => println!("aaa: {:#?}",a),
    //     GethTrace::Unknown(_) => todo!(),
    // };

    // println!("TRACE: {:#?}", smtg);

    gen_data.store();
}
