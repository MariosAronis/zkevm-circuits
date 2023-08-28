pub use benchmarks::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod benchmarks {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkExtCodeSize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkExtCodeSize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkMload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkMload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Benchmarks.Len"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSdiv"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSdiv"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("l"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Benchmarks.Len"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BENCHMARKS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\t\x16\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x06[\x02\x03\x14a\0FW\x80c\n^\x86\xCA\x14a\0vW\x80c\x9Bl\x93D\x14a\0\xA6W[`\0\x80\xFD[a\0``\x04\x806\x03\x81\x01\x90a\0[\x91\x90a\x06\xBCV[a\0\xD6V[`@Qa\0m\x91\x90a\x07\x08V[`@Q\x80\x91\x03\x90\xF3[a\0\x90`\x04\x806\x03\x81\x01\x90a\0\x8B\x91\x90a\x07\x88V[a\x01\xCBV[`@Qa\0\x9D\x91\x90a\x07\xEEV[`@Q\x80\x91\x03\x90\xF3[a\0\xC0`\x04\x806\x03\x81\x01\x90a\0\xBB\x91\x90a\x06\xBCV[a\x05\xFCV[`@Qa\0\xCD\x91\x90a\x07\x08V[`@Q\x80\x91\x03\x90\xF3[`\0`\x045`d\x81\x04`M`\0[\x82\x81\x10\x15a\x01\xC2W\x84\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x82\x05\x94P`\x01\x81\x01\x90Pa\0\xE4V[PPPP\x91\x90PV[`\0\x80`D\x90P`\0`d\x85\x85\x90Pa\x01\xE4\x91\x90a\x08gV[\x90P`\0` \x90P`\0[\x82\x81\x10\x15a\x05\xF2W\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x835;\x94P\x81\x84\x01\x93P\x80\x80a\x05\xEA\x90a\x08\x98V[\x91PPa\x01\xEFV[PPPP\x92\x91PPV[`\0`\x045`d\x81\x04`M`\0[\x82\x81\x10\x15a\x06\x85W`\0QQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQQ\x94P`\x01\x81\x01\x90Pa\x06\nV[PPPP\x91\x90PV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x06\xB3Wa\x06\xB2a\x06\x98V[[\x81\x90P\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x06\xD2Wa\x06\xD1a\x06\x8EV[[`\0a\x06\xE0\x84\x82\x85\x01a\x06\x9DV[\x91PP\x92\x91PPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x07\x02\x81a\x06\xE9V[\x82RPPV[`\0` \x82\x01\x90Pa\x07\x1D`\0\x83\x01\x84a\x06\xF9V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x07HWa\x07Ga\x07#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07eWa\x07da\x07(V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a\x07\x81Wa\x07\x80a\x07-V[[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x07\x9FWa\x07\x9Ea\x06\x8EV[[`\0\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xBDWa\x07\xBCa\x06\x93V[[a\x07\xC9\x85\x82\x86\x01a\x072V[\x92P\x92PP\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[a\x07\xE8\x81a\x07\xD5V[\x82RPPV[`\0` \x82\x01\x90Pa\x08\x03`\0\x83\x01\x84a\x07\xDFV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0a\x08r\x82a\x07\xD5V[\x91Pa\x08}\x83a\x07\xD5V[\x92P\x82a\x08\x8DWa\x08\x8Ca\x08\tV[[\x82\x82\x04\x90P\x92\x91PPV[`\0a\x08\xA3\x82a\x07\xD5V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a\x08\xD5Wa\x08\xD4a\x088V[[`\x01\x82\x01\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 @\xDC\r\xD3\xA3\xF8N\x8A1Ts<\xDASz>'\xC9\xB3\xC8\xD4\xA1\xC1z\x8B\xDC\xA9\xE3]\xE3o\xF4dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BENCHMARKS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    pub struct benchmarks<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for benchmarks<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for benchmarks<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for benchmarks<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for benchmarks<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(benchmarks)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> benchmarks<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BENCHMARKS_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                BENCHMARKS_ABI.clone(),
                BENCHMARKS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkExtCodeSize` (0x0a5e86ca) function
        pub fn check_ext_code_size(
            &self,
            addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 94, 134, 202], addresses)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkMload` (0x9b6c9344) function
        pub fn check_mload(
            &self,
            l: Len,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([155, 108, 147, 68], (l,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSdiv` (0x065b0203) function
        pub fn check_sdiv(
            &self,
            l: Len,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([6, 91, 2, 3], (l,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for benchmarks<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `checkExtCodeSize` function with signature `checkExtCodeSize(address[])` and selector `0x0a5e86ca`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "checkExtCodeSize", abi = "checkExtCodeSize(address[])")]
    pub struct CheckExtCodeSizeCall {
        pub addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `checkMload` function with signature `checkMload((uint32))` and selector `0x9b6c9344`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "checkMload", abi = "checkMload((uint32))")]
    pub struct CheckMloadCall {
        pub l: Len,
    }
    ///Container type for all input parameters for the `checkSdiv` function with signature `checkSdiv((uint32))` and selector `0x065b0203`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "checkSdiv", abi = "checkSdiv((uint32))")]
    pub struct CheckSdivCall {
        pub l: Len,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum benchmarksCalls {
        CheckExtCodeSize(CheckExtCodeSizeCall),
        CheckMload(CheckMloadCall),
        CheckSdiv(CheckSdivCall),
    }
    impl ::ethers::core::abi::AbiDecode for benchmarksCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CheckExtCodeSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CheckExtCodeSize(decoded));
            }
            if let Ok(decoded)
                = <CheckMloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckMload(decoded));
            }
            if let Ok(decoded)
                = <CheckSdivCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckSdiv(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for benchmarksCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckExtCodeSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckMload(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSdiv(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for benchmarksCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckExtCodeSize(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckMload(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSdiv(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckExtCodeSizeCall> for benchmarksCalls {
        fn from(value: CheckExtCodeSizeCall) -> Self {
            Self::CheckExtCodeSize(value)
        }
    }
    impl ::core::convert::From<CheckMloadCall> for benchmarksCalls {
        fn from(value: CheckMloadCall) -> Self {
            Self::CheckMload(value)
        }
    }
    impl ::core::convert::From<CheckSdivCall> for benchmarksCalls {
        fn from(value: CheckSdivCall) -> Self {
            Self::CheckSdiv(value)
        }
    }
    ///Container type for all return fields from the `checkExtCodeSize` function with signature `checkExtCodeSize(address[])` and selector `0x0a5e86ca`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckExtCodeSizeReturn {
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `checkMload` function with signature `checkMload((uint32))` and selector `0x9b6c9344`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckMloadReturn {
        pub r: u32,
    }
    ///Container type for all return fields from the `checkSdiv` function with signature `checkSdiv((uint32))` and selector `0x065b0203`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckSdivReturn {
        pub r: u32,
    }
    ///`Len(uint32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Len(pub u32);
}
