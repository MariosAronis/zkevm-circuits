1. Add build dependencies to cargo.toml
2. Create build.rs - compiles smart contracts and created rust bindings
3. mv all contracts directly under contracts folder and replace all benchmark ones with a single contract (BENCHMARKS.sol)
4. remove compilation steps from gen_blockchain_data.rs and rewrite deploy function
5. Submit transactions for circuit benchmarks (MLOAD for EVM and SDIV for STATE)