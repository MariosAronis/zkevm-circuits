pub use greeter::*;
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
pub mod greeter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("num"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("retrieve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("retrieve"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("retrieve_failing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("retrieve_failing"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_value"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_value"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("num"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_value_failing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_value_failing"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("num"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
    pub static GREETER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x02\x7F8\x03\x80a\x02\x7F\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\0zV[\x80`\0\x81\x90UPPa\0\xA7V[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\0W\x81a\0DV[\x81\x14a\0bW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\0t\x81a\0NV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\0\x90Wa\0\x8Fa\0?V[[`\0a\0\x9E\x84\x82\x85\x01a\0eV[\x91PP\x92\x91PPV[a\x01\xC9\x80a\0\xB6`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c!\x84\x8CF\x14a\0QW\x80c.d\xCE\xC1\x14a\0mW\x80c\xB0\xF2\xB7*\x14a\0\x8BW\x80c\xF3Avs\x14a\0\xA7W[`\0\x80\xFD[a\0k`\x04\x806\x03\x81\x01\x90a\0f\x91\x90a\x01<V[a\0\xC5V[\0[a\0ua\0\xDAV[`@Qa\0\x82\x91\x90a\x01xV[`@Q\x80\x91\x03\x90\xF3[a\0\xA5`\x04\x806\x03\x81\x01\x90a\0\xA0\x91\x90a\x01<V[a\0\xE3V[\0[a\0\xAFa\0\xEDV[`@Qa\0\xBC\x91\x90a\x01xV[`@Q\x80\x91\x03\x90\xF3[\x80`\0\x81\x90UP`\0a\0\xD7W`\0\x80\xFD[PV[`\0\x80T\x90P\x90V[\x80`\0\x81\x90UPPV[`\0\x80a\0\xF9W`\0\x80\xFD[`\0T\x90P\x90V[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x01\x19\x81a\x01\x06V[\x81\x14a\x01$W`\0\x80\xFD[PV[`\0\x815\x90Pa\x016\x81a\x01\x10V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x01RWa\x01Qa\x01\x01V[[`\0a\x01`\x84\x82\x85\x01a\x01'V[\x91PP\x92\x91PPV[a\x01r\x81a\x01\x06V[\x82RPPV[`\0` \x82\x01\x90Pa\x01\x8D`\0\x83\x01\x84a\x01iV[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x9B\xA2\x99\x84\x1E\xBE\xB8\x88\xC4\x81'\xF6\x8Ey\xEC\x1Fkz\x0B\x9F\x8EN\xC9\xFF[\x0CV\x85!-\xE1\xB0dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static GREETER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    pub struct greeter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for greeter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for greeter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for greeter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for greeter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(greeter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> greeter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GREETER_ABI.clone(),
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
                GREETER_ABI.clone(),
                GREETER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `retrieve` (0x2e64cec1) function
        pub fn retrieve(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 100, 206, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `retrieve_failing` (0xf3417673) function
        pub fn retrieve_failing(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 65, 118, 115], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_value` (0xb0f2b72a) function
        pub fn set_value(
            &self,
            num: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 242, 183, 42], num)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_value_failing` (0x21848c46) function
        pub fn set_value_failing(
            &self,
            num: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 132, 140, 70], num)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for greeter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `retrieve` function with signature `retrieve()` and selector `0x2e64cec1`
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
    #[ethcall(name = "retrieve", abi = "retrieve()")]
    pub struct RetrieveCall;
    ///Container type for all input parameters for the `retrieve_failing` function with signature `retrieve_failing()` and selector `0xf3417673`
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
    #[ethcall(name = "retrieve_failing", abi = "retrieve_failing()")]
    pub struct RetrieveFailingCall;
    ///Container type for all input parameters for the `set_value` function with signature `set_value(uint256)` and selector `0xb0f2b72a`
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
    #[ethcall(name = "set_value", abi = "set_value(uint256)")]
    pub struct SetValueCall {
        pub num: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `set_value_failing` function with signature `set_value_failing(uint256)` and selector `0x21848c46`
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
    #[ethcall(name = "set_value_failing", abi = "set_value_failing(uint256)")]
    pub struct SetValueFailingCall {
        pub num: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum greeterCalls {
        Retrieve(RetrieveCall),
        RetrieveFailing(RetrieveFailingCall),
        SetValue(SetValueCall),
        SetValueFailing(SetValueFailingCall),
    }
    impl ::ethers::core::abi::AbiDecode for greeterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <RetrieveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Retrieve(decoded));
            }
            if let Ok(decoded)
                = <RetrieveFailingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RetrieveFailing(decoded));
            }
            if let Ok(decoded)
                = <SetValueCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetValue(decoded));
            }
            if let Ok(decoded)
                = <SetValueFailingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetValueFailing(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for greeterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Retrieve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RetrieveFailing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetValueFailing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for greeterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Retrieve(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetrieveFailing(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetValueFailing(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<RetrieveCall> for greeterCalls {
        fn from(value: RetrieveCall) -> Self {
            Self::Retrieve(value)
        }
    }
    impl ::core::convert::From<RetrieveFailingCall> for greeterCalls {
        fn from(value: RetrieveFailingCall) -> Self {
            Self::RetrieveFailing(value)
        }
    }
    impl ::core::convert::From<SetValueCall> for greeterCalls {
        fn from(value: SetValueCall) -> Self {
            Self::SetValue(value)
        }
    }
    impl ::core::convert::From<SetValueFailingCall> for greeterCalls {
        fn from(value: SetValueFailingCall) -> Self {
            Self::SetValueFailing(value)
        }
    }
    ///Container type for all return fields from the `retrieve` function with signature `retrieve()` and selector `0x2e64cec1`
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
    pub struct RetrieveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `retrieve_failing` function with signature `retrieve_failing()` and selector `0xf3417673`
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
    pub struct RetrieveFailingReturn(pub ::ethers::core::types::U256);
}
